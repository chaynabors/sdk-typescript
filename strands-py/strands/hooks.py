from __future__ import annotations

from typing import Any, Callable


class HookRegistry:
    """Registry for event callbacks on an Agent."""

    def __init__(self):
        self._callbacks: dict[type, list[Callable]] = {}

    def add_callback(self, event_type: type, callback: Callable) -> None:
        self._callbacks.setdefault(event_type, []).append(callback)

    def fire(self, event: Any) -> None:
        for cb in self._callbacks.get(type(event), []):
            cb(event)


class AfterToolCallEvent:
    """Fired after a tool call completes. Set retry=True to re-invoke."""

    def __init__(self):
        self.tool_use: dict[str, Any] = {}
        self.result: dict[str, Any] = {}
        self.retry: bool = False


class AfterModelCallEvent:
    pass


class BeforeModelCallEvent:
    pass


class BeforeInvocationEvent:
    pass


class AfterInvocationEvent:
    pass


class BeforeToolCallEvent:
    pass


class AgentInitializedEvent:
    pass


class MessageAddedEvent:
    pass


class HookProvider:
    """Base class for hook providers."""

    pass
