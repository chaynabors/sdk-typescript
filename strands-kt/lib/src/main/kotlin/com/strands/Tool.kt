package com.strands

import uniffi.strands.ToolSpecConfig

fun interface ToolHandler {
    fun handle(input: String, toolUseId: String): String
}

class Tool(
    val name: String,
    val description: String,
    val inputSchema: String,
    val handler: ToolHandler,
) {
    internal fun toSpec(): ToolSpecConfig = ToolSpecConfig(
        name = name,
        description = description,
        inputSchema = inputSchema,
    )
}
