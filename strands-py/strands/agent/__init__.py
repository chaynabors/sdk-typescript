from __future__ import annotations

import json
import logging
import sys
from typing import Any

from strands._strands import Agent as _RustAgent
from strands._conversions import (
    _convert_message,
    _event_from_pyo3,
    _event_to_dict,
    _flatten_pydantic_schema,
    _resolve_model,
    _stop_reason_to_snake,
)
from strands.generated.wit_world.imports.types import (
    StreamEvent_Error,
    StreamEvent_Stop,
    StreamEvent_TextDelta,
    StreamEvent_ToolResult,
    StreamEvent_ToolUse,
)
from strands.hooks import AfterToolCallEvent, HookRegistry
from strands.tools import DecoratedTool
from strands.types.tools import ToolContext
from strands.types.exceptions import MaxTokensReachedException, _ContextOverflowError

log = logging.getLogger(__name__)


# ── AgentResult & _Metrics (formerly in _result.py) ─────────────────


class _Metrics:
    """Python-side metrics wrapper with tool_metrics support."""

    def __init__(self, latency_ms: float = 0.0, tool_metrics: list | None = None):
        self.latency_ms = latency_ms
        self.tool_metrics = tool_metrics


class AgentResult:
    """SDK-compatible result from an agent invocation."""

    def __init__(
        self,
        text: str,
        stop_reason: str,
        usage: Any = None,
        metrics: Any = None,
        structured_output: Any = None,
        message: dict | None = None,
    ):
        self.text = text
        self.stop_reason = stop_reason
        self.usage = usage
        self.metrics = metrics
        self.structured_output = structured_output
        self.message: dict[str, Any] = message or {
            "role": "assistant",
            "content": [{"text": text}],
        }

    def __str__(self) -> str:
        return self.text

    def __repr__(self) -> str:
        return f"AgentResult(stop_reason={self.stop_reason!r}, text={self.text[:80]!r})"


# ── Tool proxies ────────────────────────────────────────────────────


class _ToolRegistryProxy:
    """Proxy for agent.tool_registry with mutable registry/tool_config."""

    def __init__(self, registry: dict[str, Any]):
        self.registry = registry
        self.tool_config: dict[str, Any] = {}


class _ToolProxy:
    def __init__(self, tools: dict[str, Any], agent: Any = None):
        self._tools = tools
        self._agent = agent

    def __getattr__(self, name: str):
        if name.startswith("_"):
            raise AttributeError(name)
        if name not in self._tools:
            raise AttributeError(f"No tool named '{name}'")
        entry = self._tools[name]
        agent = self._agent

        def invoke(**kwargs):
            import uuid

            tool_use_id = f"tooluse_{uuid.uuid4().hex[:24]}"
            while True:
                ctx_param = entry.get("context_param")
                call_kwargs = dict(kwargs)
                if ctx_param and agent is not None:
                    call_kwargs[ctx_param] = ToolContext(
                        tool_use={"toolUseId": tool_use_id},
                        agent=agent,
                    )
                try:
                    raw = entry["callable"](**call_kwargs)
                    if isinstance(raw, dict) and "status" in raw and "content" in raw:
                        result = raw
                    else:
                        result = {"status": "success", "content": [{"text": str(raw)}]}
                except Exception as exc:
                    result = {"status": "error", "content": [{"text": str(exc)}]}

                if agent is not None and hasattr(agent, "hooks"):
                    event = AfterToolCallEvent()
                    event.tool_use = {"toolUseId": tool_use_id}
                    event.result = result
                    event.retry = False
                    agent.hooks.fire(event)
                    if event.retry:
                        continue
                return result

        return invoke


# ── Agent ───────────────────────────────────────────────────────────


class Agent:
    """SDK-compatible Agent wrapping the WASM-hosted runtime.

    Usage matches the existing Python SDK::

        agent = Agent(tools=[my_tool], system_prompt="Be helpful.")
        result = agent("Hello!")
        print(result)
    """

    def __init__(
        self,
        *,
        model=None,
        system_prompt: str | None = None,
        system_prompt_blocks=None,
        tools: list | None = None,
        messages: list | None = None,
        callback_handler=None,
        load_tools_from_directory: bool = False,
        printer: bool = True,
        structured_output_model: type | None = None,
        **kwargs,
    ):
        if kwargs:
            log.debug("ignoring unknown kwargs: %s", list(kwargs.keys()))

        model_dict = _resolve_model(model)

        self._tool_map: dict[str, Any] = {}
        self.state: dict[str, Any] = {}
        self.hooks = HookRegistry()
        self._default_structured_output_model = structured_output_model
        rust_tools = None

        if tools is not None:
            rust_tools = []
            for t in tools:
                if isinstance(t, DecoratedTool):
                    self._tool_map[t.tool_name] = {
                        "callable": t._func,
                        "spec": t.tool_spec,
                        "context_param": t._context_param,
                    }
                    rust_tools.append(
                        {
                            "name": t.tool_name,
                            "description": t.tool_spec["description"],
                            "inputSchema": t.tool_spec.get("inputSchema", {}),
                            "handler": t._make_handler(agent_ref=self),
                        }
                    )
                elif isinstance(t, dict):
                    if "handler" in t:
                        self._tool_map[t["name"]] = {
                            "callable": t["handler"],
                            "spec": {k: v for k, v in t.items() if k != "handler"},
                        }
                    clean = {k: v for k, v in t.items() if k != "handler"}
                    rust_tools.append(clean)

        sp_blocks = None
        if system_prompt_blocks is not None:
            sp_blocks = (
                system_prompt_blocks
                if isinstance(system_prompt_blocks, str)
                else json.dumps(system_prompt_blocks)
            )

        self._load_tools_from_directory = load_tools_from_directory
        self._tools_dir_mtimes: dict[str, float] = {}

        if load_tools_from_directory:
            self._scan_tools_directory()

        self._rust_agent = _RustAgent(
            model=model_dict,
            system_prompt=system_prompt,
            system_prompt_blocks=sp_blocks,
            tools=rust_tools,
        )
        self._printer = printer

        if messages is not None:
            self._rust_agent.set_messages(json.dumps(messages))

    def _scan_tools_directory(self) -> None:
        """Scan ./tools/ for .py files with @tool-decorated functions."""
        import importlib.util
        from pathlib import Path

        tools_dir = Path.cwd() / "tools"
        if not tools_dir.is_dir():
            return

        for py_file in tools_dir.glob("*.py"):
            mtime = py_file.stat().st_mtime
            name = py_file.stem
            if name in self._tools_dir_mtimes and self._tools_dir_mtimes[name] >= mtime:
                continue
            self._tools_dir_mtimes[name] = mtime
            try:
                spec = importlib.util.spec_from_file_location(f"tools.{name}", py_file)
                mod = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(mod)
                for attr_name in dir(mod):
                    obj = getattr(mod, attr_name)
                    if isinstance(obj, DecoratedTool):
                        self._tool_map[obj.tool_name] = {
                            "callable": obj._func,
                            "spec": obj.tool_spec,
                            "context_param": obj._context_param,
                        }
            except Exception:
                log.warning("failed to load tool from %s", py_file, exc_info=True)

    @property
    def messages(self) -> list[dict[str, Any]]:
        raw = json.loads(self._rust_agent.get_messages())
        return [_convert_message(msg) for msg in raw]

    @messages.setter
    def messages(self, value: list[dict[str, Any]]) -> None:
        self._rust_agent.set_messages(json.dumps(value))

    @property
    def tool(self) -> _ToolProxy:
        if self._load_tools_from_directory:
            self._scan_tools_directory()
        return _ToolProxy(self._tool_map, agent=self)

    @property
    def tool_names(self) -> list[str]:
        if self._load_tools_from_directory:
            self._scan_tools_directory()
        return list(self._tool_map.keys())

    @property
    def tool_registry(self) -> _ToolRegistryProxy:
        return _ToolRegistryProxy(self._tool_map)

    async def _consume_stream_async(
        self,
        prompt: str,
        *,
        tools=None,
        tool_choice=None,
    ) -> tuple[list[str], str, Any, Any]:
        import time as _time

        text_parts: list[str] = []
        stop_reason = "end_turn"
        usage = None
        metrics = None
        tool_metrics: list[dict[str, Any]] = []
        pending_tool_start: dict[str, float] = {}

        stream = await self._rust_agent.start_stream(
            prompt, tools=tools, tool_choice=tool_choice
        )
        try:
            while True:
                batch = await self._rust_agent.next_events(stream)
                if batch is None:
                    break
                for raw_event in batch:
                    event = _event_from_pyo3(raw_event)
                    if event is None:
                        continue

                    if isinstance(event, StreamEvent_TextDelta):
                        text_parts.append(event.value)
                        if self._printer:
                            print(event.value, end="", flush=True)

                    elif isinstance(event, StreamEvent_Stop):
                        stop_reason = _stop_reason_to_snake(event.value.reason)
                        usage = event.value.usage
                        metrics = event.value.metrics
                        if self._printer and text_parts:
                            print()

                    elif isinstance(event, StreamEvent_ToolUse):
                        pending_tool_start[event.value.tool_use_id] = _time.monotonic()

                    elif isinstance(event, StreamEvent_ToolResult):
                        tid = event.value.tool_use_id
                        if tid in pending_tool_start:
                            duration = _time.monotonic() - pending_tool_start.pop(tid)
                            tool_metrics.append(
                                {
                                    "toolUseId": tid,
                                    "duration": duration,
                                    "status": event.value.status,
                                }
                            )

                    elif isinstance(event, StreamEvent_Error):
                        err_msg = event.value
                        if (
                            "context" in err_msg.lower()
                            and "exceeded" in err_msg.lower()
                        ):
                            raise _ContextOverflowError(err_msg)
                        if self._printer:
                            print(f"\n[error: {err_msg}]", file=sys.stderr)

        finally:
            await self._rust_agent.close_stream(stream)

        if stop_reason == "model_context_window_exceeded":
            raise _ContextOverflowError("context window exceeded")

        latency = metrics.latency_ms if metrics else 0.0
        py_metrics = _Metrics(
            latency_ms=latency,
            tool_metrics=tool_metrics or None,
        )
        return text_parts, stop_reason, usage, py_metrics

    async def _call_async(self, prompt: str, **kwargs) -> AgentResult:
        structured_output_model = kwargs.pop("structured_output_model", None)
        so_model = structured_output_model or self._default_structured_output_model

        if so_model is not None:
            return await self._call_with_structured_output_async(prompt, so_model)

        try:
            text_parts, stop_reason, usage, metrics = await self._consume_stream_async(
                prompt
            )
        except _ContextOverflowError:
            msgs = self.messages
            if len(msgs) > 2:
                self.messages = msgs[-2:]
            text_parts, stop_reason, usage, metrics = await self._consume_stream_async(
                prompt
            )

        if stop_reason == "max_tokens":
            msgs = self.messages
            msgs.append(
                {
                    "role": "user",
                    "content": [
                        {
                            "text": "tool use was incomplete due to maximum token limits being reached"
                        }
                    ],
                }
            )
            self.messages = msgs
            raise MaxTokensReachedException("max tokens reached")

        return AgentResult(
            text="".join(text_parts),
            stop_reason=stop_reason,
            usage=usage,
            metrics=metrics,
        )

    async def _call_with_structured_output_async(
        self, prompt: str, so_model: type
    ) -> AgentResult:
        so_tool_name = so_model.__name__
        schema = _flatten_pydantic_schema(so_model.model_json_schema())
        so_tool_spec = {
            "name": so_tool_name,
            "description": (getattr(so_model, "__doc__", None) or so_tool_name)
            + " -- You MUST call this tool to return structured output.",
            "inputSchema": schema,
        }
        captured: dict[str, Any] = {"result": None}

        def so_handler(input_json: str, tool_use_id: str = "") -> str:
            data = json.loads(input_json)
            try:
                validated = so_model(**data)
                captured["result"] = validated
                return json.dumps(
                    {
                        "status": "success",
                        "content": [{"text": json.dumps(data)}],
                    }
                )
            except Exception as exc:
                raise ValueError(f"Validation error: {exc}") from exc

        self._rust_agent._register_handler(so_tool_name, so_handler)
        try:
            existing_tools = [
                entry["spec"] for entry in self._tool_map.values() if "spec" in entry
            ]
            all_tools = existing_tools + [so_tool_spec]
            text_parts, stop_reason, usage, metrics = await self._consume_stream_async(
                prompt, tools=all_tools
            )

            if captured["result"] is None and stop_reason != "max_tokens":
                (
                    text_parts,
                    stop_reason,
                    usage,
                    metrics,
                ) = await self._consume_stream_async(
                    json.dumps(
                        [
                            {
                                "text": "You must format the previous response as structured output. "
                                f"Call the {so_tool_name} tool now."
                            }
                        ]
                    ),
                    tools=[so_tool_spec],
                    tool_choice=json.dumps({"any": {}}),
                )

            if stop_reason == "max_tokens":
                raise MaxTokensReachedException("max tokens reached")

            return AgentResult(
                text="".join(text_parts),
                stop_reason=stop_reason,
                usage=usage,
                metrics=metrics,
                structured_output=captured["result"],
            )
        finally:
            self._rust_agent._unregister_handler(so_tool_name)

    def __call__(self, prompt=None, **kwargs) -> AgentResult:
        import asyncio

        if self._load_tools_from_directory:
            self._scan_tools_directory()
        if prompt is None:
            prompt = ""
        if isinstance(prompt, list):
            prompt = json.dumps(prompt)
        prompt = str(prompt)
        return asyncio.run(self._call_async(prompt, **kwargs))

    def invoke(self, prompt: str) -> AgentResult:
        old = self._printer
        self._printer = False
        try:
            return self(prompt)
        finally:
            self._printer = old

    async def invoke_async(self, prompt: str, **kwargs) -> AgentResult:
        return await self._call_async(str(prompt), **kwargs)

    async def stream_async(self, prompt, **kwargs):
        structured_output_model = kwargs.pop("structured_output_model", None)
        so_model = structured_output_model or self._default_structured_output_model

        if so_model is not None:
            result = await self._call_async(
                str(prompt), structured_output_model=so_model
            )
            yield {"result": result}
            return

        stream = await self._rust_agent.start_stream(str(prompt))
        try:
            while True:
                batch = await self._rust_agent.next_events(stream)
                if batch is None:
                    break
                for event in batch:
                    yield _event_to_dict(event)
        finally:
            await self._rust_agent.close_stream(stream)

    def get_messages(self) -> str:
        return self._rust_agent.get_messages()

    def set_messages(self, json_str: str) -> None:
        self._rust_agent.set_messages(json_str)


# Re-export for test compatibility
from strands.agent.conversation_manager import NullConversationManager  # noqa: E402

__all__ = ["Agent", "AgentResult", "NullConversationManager"]
