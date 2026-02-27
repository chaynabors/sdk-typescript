package com.strands

import uniffi.strands.StreamEvent as RawStreamEvent

sealed class StreamEvent {
    data class TextDelta(val text: String) : StreamEvent()
    data class ToolUse(val name: String, val toolUseId: String, val input: String) : StreamEvent()
    data class ToolResult(val toolUseId: String, val status: String, val content: String) : StreamEvent()
    data class Metadata(val usage: Usage?, val metrics: Metrics?) : StreamEvent()
    data class Stop(val reason: StopReason, val usage: Usage?, val metrics: Metrics?) : StreamEvent()
    data class Error(val message: String) : StreamEvent()
    data class Interrupt(val payload: String) : StreamEvent()

    companion object {
        internal fun from(raw: RawStreamEvent): StreamEvent? = when (raw.kind) {
            "text-delta" -> TextDelta(raw.textDelta ?: "")
            "tool-use" -> {
                val tu = raw.toolUse!!
                ToolUse(tu.name, tu.toolUseId, tu.input)
            }
            "tool-result" -> {
                val tr = raw.toolResult!!
                ToolResult(tr.toolUseId, tr.status, tr.content)
            }
            "metadata" -> {
                val m = raw.metadata
                Metadata(
                    m?.usage?.let { Usage.from(it) },
                    m?.metrics?.let { Metrics.from(it) },
                )
            }
            "stop" -> {
                val s = raw.stop!!
                Stop(
                    StopReason.from(s.reason.value),
                    s.usage?.let { Usage.from(it) },
                    s.metrics?.let { Metrics.from(it) },
                )
            }
            "error" -> Error(raw.error ?: "unknown error")
            "interrupt" -> Interrupt(raw.interrupt ?: "")
            "lifecycle" -> null // internal event, not surfaced to user
            else -> null
        }
    }
}
