package com.strands

import uniffi.strands.ModelConfigInput

sealed class ModelConfig {
    data class Bedrock(
        val modelId: String = "",
        val region: String? = null,
        val accessKeyId: String? = null,
        val secretAccessKey: String? = null,
        val sessionToken: String? = null,
    ) : ModelConfig()

    data class Anthropic(
        val modelId: String? = null,
        val apiKey: String? = null,
    ) : ModelConfig()

    data class OpenAI(
        val modelId: String? = null,
        val apiKey: String? = null,
    ) : ModelConfig()

    data class Gemini(
        val modelId: String? = null,
        val apiKey: String? = null,
    ) : ModelConfig()

    internal fun toNative(): ModelConfigInput = when (this) {
        is Bedrock -> ModelConfigInput(
            provider = "bedrock",
            modelId = modelId.ifEmpty { null },
            apiKey = null,
            region = region,
            accessKeyId = accessKeyId,
            secretAccessKey = secretAccessKey,
            sessionToken = sessionToken,
            additionalConfig = null,
        )
        is Anthropic -> ModelConfigInput(
            provider = "anthropic",
            modelId = modelId,
            apiKey = apiKey,
            region = null,
            accessKeyId = null,
            secretAccessKey = null,
            sessionToken = null,
            additionalConfig = null,
        )
        is OpenAI -> ModelConfigInput(
            provider = "openai",
            modelId = modelId,
            apiKey = apiKey,
            region = null,
            accessKeyId = null,
            secretAccessKey = null,
            sessionToken = null,
            additionalConfig = null,
        )
        is Gemini -> ModelConfigInput(
            provider = "gemini",
            modelId = modelId,
            apiKey = apiKey,
            region = null,
            accessKeyId = null,
            secretAccessKey = null,
            sessionToken = null,
            additionalConfig = null,
        )
    }

    companion object {
        @JvmStatic @JvmOverloads fun bedrock(modelId: String = ""): Bedrock = Bedrock(modelId = modelId)
        @JvmStatic @JvmOverloads fun anthropic(modelId: String? = null, apiKey: String? = null): Anthropic = Anthropic(modelId, apiKey)
        @JvmStatic @JvmOverloads fun openai(modelId: String? = null, apiKey: String? = null): OpenAI = OpenAI(modelId, apiKey)
        @JvmStatic @JvmOverloads fun gemini(modelId: String? = null, apiKey: String? = null): Gemini = Gemini(modelId, apiKey)
    }
}
