package com.strands

import java.util.concurrent.CompletableFuture
import java.util.logging.Level
import java.util.logging.Logger
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.future.future
import kotlinx.coroutines.runBlocking
import uniffi.strands.Agent as NativeAgent
import uniffi.strands.LogHandler as NativeLogHandler
import uniffi.strands.ToolDispatcher as NativeToolDispatcher

class Agent @JvmOverloads constructor(
    model: ModelConfig? = null,
    systemPrompt: String? = null,
    tools: List<Tool>? = null,
) : AutoCloseable {

    private val toolMap = mutableMapOf<String, ToolHandler>()
    private val native: NativeAgent
    private val scope = CoroutineScope(Dispatchers.Default)

    init {
        val specs = tools?.map { tool ->
            toolMap[tool.name] = tool.handler
            tool.toSpec()
        }

        val dispatcher: NativeToolDispatcher? = if (toolMap.isNotEmpty()) {
            object : NativeToolDispatcher {
                override fun callTool(name: String, input: String, toolUseId: String): String {
                    val handler = toolMap[name]
                        ?: throw IllegalArgumentException("unknown tool: $name")
                    return handler.handle(input, toolUseId)
                }
            }
        } else null

        val logger = object : NativeLogHandler {
            private val log = Logger.getLogger("com.strands")
            override fun log(level: String, message: String, context: String?) {
                val jLevel = when (level) {
                    "error" -> Level.SEVERE
                    "warn" -> Level.WARNING
                    "info" -> Level.INFO
                    "debug" -> Level.FINE
                    else -> Level.FINEST
                }
                log.log(jLevel, if (context != null) "$message | $context" else message)
            }
        }

        native = NativeAgent(
            model = model?.toNative(),
            systemPrompt = systemPrompt,
            tools = specs,
            toolDispatcher = dispatcher,
            logHandler = logger,
        )
    }

    fun stream(input: String): Flow<StreamEvent> = flow {
        val handle = native.startStream(input)
        try {
            while (true) {
                val batch = native.nextEvents(handle) ?: break
                for (raw in batch) {
                    StreamEvent.from(raw)?.let { emit(it) }
                }
            }
        } finally {
            native.closeStream(handle)
        }
    }

    suspend fun invoke(input: String): AgentResult {
        val textParts = mutableListOf<String>()
        var stopReason = StopReason.EndTurn
        var usage: Usage? = null
        var metrics: Metrics? = null

        val handle = native.startStream(input)
        try {
            while (true) {
                val batch = native.nextEvents(handle) ?: break
                for (raw in batch) {
                    when (val event = StreamEvent.from(raw) ?: continue) {
                        is StreamEvent.TextDelta -> textParts.add(event.text)
                        is StreamEvent.Stop -> {
                            stopReason = event.reason
                            usage = event.usage
                            metrics = event.metrics
                        }
                        is StreamEvent.Error -> throw RuntimeException(event.message)
                        else -> {}
                    }
                }
            }
        } finally {
            native.closeStream(handle)
        }

        return AgentResult(
            text = textParts.joinToString(""),
            stopReason = stopReason,
            usage = usage,
            metrics = metrics,
        )
    }

    fun invokeAsync(input: String): CompletableFuture<AgentResult> =
        scope.future { invoke(input) }

    @JvmName("invokeBlocking")
    fun invokeBlocking(input: String): AgentResult = runBlocking { invoke(input) }

    fun forEachEvent(input: String, consumer: java.util.function.Consumer<StreamEvent>): CompletableFuture<Void> =
        scope.future { stream(input).collect { consumer.accept(it) } }
            .thenApply { null }

    var messages: String
        get() = native.getMessages()
        set(value) = native.setMessages(value)

    override fun close() {
        native.close()
    }
}
