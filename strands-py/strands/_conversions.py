"""Conversion between PyO3 flat types, generated WIT dataclasses, and legacy dicts.

Three representations coexist:
  1. PyO3 flat types (StreamEvent_ with kind="text-delta") — cross the FFI boundary
  2. Generated WIT dataclasses (StreamEvent_TextDelta) — typed, used internally
  3. Legacy dicts ({"event": {...}}) — used by stream_async() for SDK compat

Functions prefixed _event_from_pyo3 / _usage_from_pyo3 convert 1→2.
Functions prefixed _event_to_dict convert 1→3.
"""

from __future__ import annotations

import json
import logging
from typing import Any

from strands.generated.wit_world.imports.types import (
    MetadataEvent,
    Metrics,
    StopData,
    StopReason,
    StreamEvent,
    StreamEvent_Error,
    StreamEvent_Interrupt,
    StreamEvent_Metadata,
    StreamEvent_Stop,
    StreamEvent_TextDelta,
    StreamEvent_ToolResult,
    StreamEvent_ToolUse,
    ToolResultEvent,
    ToolUseEvent,
    Usage,
)

log = logging.getLogger(__name__)


# ── PyO3 string → WIT StopReason enum ──────────────────────────────

_STOP_REASON_MAP: dict[str, StopReason] = {
    "end-turn": StopReason.END_TURN,
    "tool-use": StopReason.TOOL_USE,
    "max-tokens": StopReason.MAX_TOKENS,
    "error": StopReason.ERROR,
    "content-filtered": StopReason.CONTENT_FILTERED,
    "guardrail-intervened": StopReason.GUARDRAIL_INTERVENED,
    "stop-sequence": StopReason.STOP_SEQUENCE,
    "model-context-window-exceeded": StopReason.MODEL_CONTEXT_WINDOW_EXCEEDED,
    "cancelled": StopReason.CANCELLED,
}


def _stop_reason_from_pyo3(pyo3_stop: Any) -> StopReason:
    """Convert a PyO3 StopReason_ (string .value) to a generated StopReason enum."""
    if pyo3_stop and hasattr(pyo3_stop, "value"):
        return _STOP_REASON_MAP.get(pyo3_stop.value, StopReason.ERROR)
    return StopReason.END_TURN


def _stop_reason_to_snake(reason: StopReason) -> str:
    """Convert a WIT StopReason enum to the snake_case string the Python SDK uses."""
    return reason.name.lower()


def _stop_reason_str(stop: Any) -> str:
    """Convert a PyO3 stop payload to a snake_case string. Legacy helper."""
    if stop and stop.reason:
        return stop.reason.value.replace("-", "_")
    return "end_turn"


# ── PyO3 flat types → generated WIT dataclasses ────────────────────


def _usage_from_pyo3(u: Any) -> Usage | None:
    """Convert PyO3 Usage_ to generated Usage dataclass."""
    if u is None:
        return None
    return Usage(
        input_tokens=u.input_tokens,
        output_tokens=u.output_tokens,
        total_tokens=u.total_tokens,
        cache_read_input_tokens=u.cache_read_input_tokens,
        cache_write_input_tokens=u.cache_write_input_tokens,
    )


def _metrics_from_pyo3(m: Any) -> Metrics | None:
    """Convert PyO3 Metrics_ to generated Metrics dataclass."""
    if m is None:
        return None
    return Metrics(latency_ms=m.latency_ms)


def _event_from_pyo3(event: Any) -> StreamEvent | None:
    """Convert a PyO3 StreamEvent_ (flat struct) to a generated StreamEvent (union).

    Returns None for unrecognized event kinds.
    """
    kind = event.kind

    if kind == "text-delta":
        return StreamEvent_TextDelta(value=event.text_delta or "")

    if kind == "stop":
        stop = event.stop
        reason = _stop_reason_from_pyo3(stop.reason if stop else None)
        return StreamEvent_Stop(
            value=StopData(
                reason=reason,
                usage=_usage_from_pyo3(stop.usage) if stop else None,
                metrics=_metrics_from_pyo3(stop.metrics) if stop else None,
            )
        )

    if kind == "tool-use":
        tu = event.tool_use
        if tu:
            return StreamEvent_ToolUse(
                value=ToolUseEvent(
                    name=tu.name,
                    tool_use_id=tu.tool_use_id,
                    input=tu.input,
                )
            )
        return None

    if kind == "tool-result":
        tr = event.tool_result
        if tr:
            return StreamEvent_ToolResult(
                value=ToolResultEvent(
                    tool_use_id=tr.tool_use_id,
                    status=tr.status,
                    content=tr.content,
                )
            )
        return None

    if kind == "metadata":
        me = event.metadata
        if me:
            return StreamEvent_Metadata(
                value=MetadataEvent(
                    usage=_usage_from_pyo3(me.usage),
                    metrics=_metrics_from_pyo3(me.metrics),
                )
            )
        return None

    if kind == "error":
        return StreamEvent_Error(value=event.error or "")

    if kind == "interrupt":
        return StreamEvent_Interrupt(value=event.interrupt or "")

    log.warning("unknown stream event kind: %s", kind)
    return None


# ── Legacy dict conversion (for stream_async compatibility) ─────────


def _event_to_dict(event: Any) -> dict[str, Any]:
    """Convert a Rust StreamEvent into the dict format the Python SDK expects.

    Returns a plain dict. The "stop" branch returns a partial result dict —
    the caller is responsible for filling in the accumulated text.
    """
    from strands.agent import AgentResult

    if event.kind == "text-delta":
        return {
            "event": {"contentBlockDelta": {"delta": {"text": event.text_delta or ""}}}
        }

    if event.kind == "stop":
        stop_reason = _stop_reason_str(event.stop)
        usage = event.stop.usage if event.stop else None
        metrics = event.stop.metrics if event.stop else None
        return {
            "result": AgentResult(
                text="", stop_reason=stop_reason, usage=usage, metrics=metrics
            )
        }

    if event.kind == "tool-use":
        tu = event.tool_use
        tool_use_data = (
            {
                "name": tu.name,
                "toolUseId": tu.tool_use_id,
                "input": json.loads(tu.input) if tu.input else {},
            }
            if tu
            else {}
        )
        return {
            "event": {
                "contentBlockStart": {
                    "contentBlock": {"type": "tool_use", **tool_use_data}
                }
            }
        }

    if event.kind == "tool-result":
        tr = event.tool_result
        tool_result_data = (
            {
                "toolUseId": tr.tool_use_id,
                "status": tr.status,
                "content": json.loads(tr.content) if tr.content else [],
            }
            if tr
            else {}
        )
        return {"event": {"toolResult": tool_result_data}}

    if event.kind == "metadata":
        me = event.metadata
        metadata: dict[str, Any] = {}
        if me:
            if me.usage:
                metadata["usage"] = {
                    "inputTokens": me.usage.input_tokens,
                    "outputTokens": me.usage.output_tokens,
                    "totalTokens": me.usage.total_tokens,
                    "cacheReadInputTokens": me.usage.cache_read_input_tokens,
                    "cacheWriteInputTokens": me.usage.cache_write_input_tokens,
                }
            if me.metrics:
                metadata["metrics"] = {"latencyMs": me.metrics.latency_ms}
        return {"event": {"metadata": metadata}}

    if event.kind == "error":
        return {"error": event.error}

    log.warning("unknown stream event kind: %s", event.kind)
    return {}


# ── Message format conversion (TS SDK ↔ Python SDK) ────────────────


def _convert_message(msg: dict[str, Any]) -> dict[str, Any]:
    """Convert a single message from TS SDK format to Python SDK format."""
    if "content" not in msg:
        return msg
    return {**msg, "content": [_convert_block(b) for b in msg["content"]]}


def _convert_block(block: dict[str, Any]) -> dict[str, Any]:
    """Convert a content block from TS SDK format to Python SDK format."""
    block_type = block.get("type")
    if block_type == "textBlock":
        return {"text": block.get("text", "")}
    if block_type == "toolUseBlock":
        return {
            "toolUse": {
                "name": block.get("name", ""),
                "toolUseId": block.get("toolUseId", ""),
                "input": block.get("input", {}),
            }
        }
    if block_type == "toolResultBlock":
        return {
            "toolResult": {
                "toolUseId": block.get("toolUseId", ""),
                "status": block.get("status", "success"),
                "content": _unwrap_tool_content(block.get("content", [])),
            }
        }
    return block


def _unwrap_tool_content(content: list[Any]) -> list[dict[str, Any]]:
    """Unwrap TS SDK tool result content to Python SDK format."""
    result: list[dict[str, Any]] = []
    for item in content:
        if not isinstance(item, dict):
            result.append(item)
            continue
        item_type = item.get("type")
        if item_type == "jsonBlock":
            json_val = item.get("json", {})
            if isinstance(json_val, dict) and "$value" in json_val:
                for inner in json_val["$value"]:
                    if isinstance(inner, dict):
                        result.append(inner)
                    else:
                        result.append({"text": str(inner)})
            else:
                result.append({"json": json_val})
        elif item_type == "textBlock":
            result.append({"text": item.get("text", "")})
        else:
            result.append(item)
    return result


def _flatten_pydantic_schema(schema: dict[str, Any]) -> dict[str, Any]:
    """Flatten a pydantic JSON schema by resolving all $ref/$defs inline."""
    defs = schema.get("$defs", {})

    def resolve(obj: Any) -> Any:
        if not isinstance(obj, dict):
            return obj
        if "$ref" in obj:
            ref_name = obj["$ref"].rsplit("/", 1)[-1]
            return resolve(defs.get(ref_name, {}))
        return {k: resolve(v) for k, v in obj.items() if k != "$defs"}

    resolved = resolve(schema)
    resolved.pop("$defs", None)
    return resolved


def _resolve_model(model: Any) -> dict[str, Any] | None:
    """Normalize a model argument into a config dict (or None for default)."""
    if model is None:
        return None
    if isinstance(model, dict):
        return model
    if isinstance(model, str):
        return {"provider": "bedrock", "model_id": model}
    if hasattr(model, "_to_config_dict"):
        return model._to_config_dict()
    return None
