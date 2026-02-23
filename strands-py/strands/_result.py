"""Backward-compat shim — AgentResult and _Metrics now live in strands.agent."""

from strands.agent import AgentResult, _Metrics

__all__ = ["AgentResult", "_Metrics"]
