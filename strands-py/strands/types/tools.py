from __future__ import annotations

from typing import Any


class ToolContext:
    """Placeholder -- ToolContext is not yet bridged across the WASM boundary."""

    def __init__(self, tool_use=None, agent=None, invocation_state=None):
        self.tool_use = tool_use or {}
        self.agent = agent
        self.invocation_state = invocation_state or {}


# Type aliases matching the existing SDK.
ToolResult = dict[str, Any]
ToolSpec = dict[str, Any]
