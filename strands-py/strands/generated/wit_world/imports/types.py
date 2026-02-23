# @generated from wit/agent.wit -- do not edit
from typing import (
    TypeVar,
    Generic,
    Union,
    Optional,
    Protocol,
    Tuple,
    List,
    Any,
    Self,
    Callable,
)
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from strands.generated.componentize_py_types import Result, Ok, Err, Some


class StopReason(Enum):
    END_TURN = 0
    TOOL_USE = 1
    MAX_TOKENS = 2
    ERROR = 3
    CONTENT_FILTERED = 4
    GUARDRAIL_INTERVENED = 5
    STOP_SEQUENCE = 6
    MODEL_CONTEXT_WINDOW_EXCEEDED = 7
    CANCELLED = 8


@dataclass
class Usage:
    input_tokens: int
    output_tokens: int
    total_tokens: int
    cache_read_input_tokens: Optional[int]
    cache_write_input_tokens: Optional[int]


@dataclass
class Metrics:
    latency_ms: float


@dataclass
class MetadataEvent:
    usage: Optional[Usage]
    metrics: Optional[Metrics]


@dataclass
class ToolUseEvent:
    name: str
    tool_use_id: str
    input: str


@dataclass
class ToolResultEvent:
    tool_use_id: str
    status: str
    content: str


@dataclass
class ToolSpec:
    name: str
    description: str
    input_schema: str


@dataclass
class StopData:
    reason: StopReason
    usage: Optional[Usage]
    metrics: Optional[Metrics]


@dataclass
class StreamEvent_TextDelta:
    value: str


@dataclass
class StreamEvent_ToolUse:
    value: ToolUseEvent


@dataclass
class StreamEvent_ToolResult:
    value: ToolResultEvent


@dataclass
class StreamEvent_Metadata:
    value: MetadataEvent


@dataclass
class StreamEvent_Stop:
    value: StopData


@dataclass
class StreamEvent_Error:
    value: str


@dataclass
class StreamEvent_Interrupt:
    value: str


StreamEvent = Union[
    StreamEvent_TextDelta,
    StreamEvent_ToolUse,
    StreamEvent_ToolResult,
    StreamEvent_Metadata,
    StreamEvent_Stop,
    StreamEvent_Error,
    StreamEvent_Interrupt,
]


@dataclass
class AnthropicConfig:
    model_id: Optional[str]
    api_key: Optional[str]


@dataclass
class BedrockConfig:
    model_id: str
    region: Optional[str]
    access_key_id: Optional[str]
    secret_access_key: Optional[str]
    session_token: Optional[str]


@dataclass
class ModelConfig_Anthropic:
    value: AnthropicConfig


@dataclass
class ModelConfig_Bedrock:
    value: BedrockConfig


ModelConfig = Union[ModelConfig_Anthropic, ModelConfig_Bedrock]


@dataclass
class ModelParams:
    max_tokens: Optional[int]
    temperature: Optional[float]
    top_p: Optional[float]


@dataclass
class AgentConfig:
    model: Optional[ModelConfig]
    model_params: Optional[ModelParams]
    system_prompt: Optional[str]
    system_prompt_blocks: Optional[str]
    tools: Optional[List[ToolSpec]]
    trace_context: Optional[str]


@dataclass
class CallToolArgs:
    name: str
    input: str
    tool_use_id: str


@dataclass
class CallToolsArgs:
    calls: List[CallToolArgs]


@dataclass
class StreamArgs:
    input: str
    tools: Optional[List[ToolSpec]]
    tool_choice: Optional[str]


@dataclass
class RespondArgs:
    payload: str


@dataclass
class SetMessagesArgs:
    json: str
