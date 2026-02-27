package com.strands

import uniffi.strands.Usage as RawUsage
import uniffi.strands.Metrics as RawMetrics

enum class StopReason {
    EndTurn,
    ToolUse,
    MaxTokens,
    Error,
    ContentFiltered,
    GuardrailIntervened,
    StopSequence,
    ModelContextWindowExceeded,
    Cancelled;

    companion object {
        fun from(value: String): StopReason = when (value) {
            "end_turn", "end-turn" -> EndTurn
            "tool_use", "tool-use" -> ToolUse
            "max_tokens", "max-tokens" -> MaxTokens
            "error" -> Error
            "content_filtered", "content-filtered" -> ContentFiltered
            "guardrail_intervened", "guardrail-intervened" -> GuardrailIntervened
            "stop_sequence", "stop-sequence" -> StopSequence
            "model_context_window_exceeded", "model-context-window-exceeded" -> ModelContextWindowExceeded
            "cancelled" -> Cancelled
            else -> Error
        }
    }
}

data class Usage(
    val inputTokens: Int,
    val outputTokens: Int,
    val totalTokens: Int,
    val cacheReadInputTokens: Int?,
    val cacheWriteInputTokens: Int?,
) {
    companion object {
        internal fun from(raw: RawUsage): Usage = Usage(
            inputTokens = raw.inputTokens,
            outputTokens = raw.outputTokens,
            totalTokens = raw.totalTokens,
            cacheReadInputTokens = raw.cacheReadInputTokens,
            cacheWriteInputTokens = raw.cacheWriteInputTokens,
        )
    }
}

data class Metrics(
    val latencyMs: Double,
) {
    companion object {
        internal fun from(raw: RawMetrics): Metrics = Metrics(
            latencyMs = raw.latencyMs,
        )
    }
}

data class AgentResult(
    val text: String,
    val stopReason: StopReason,
    val usage: Usage?,
    val metrics: Metrics?,
) {
    override fun toString(): String = text
}
