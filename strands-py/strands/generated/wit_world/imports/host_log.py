# @generated from wit/agent.wit -- do not edit
"""Structured logging from guest to host."""

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


class LogLevel(Enum):
    TRACE = 0
    DEBUG = 1
    INFO = 2
    WARN = 3
    ERROR = 4


@dataclass
class LogEntry:
    level: LogLevel
    message: str
    context: Optional[str]


def log(entry: LogEntry) -> None:
    """Emit a structured log entry visible to the host."""
    raise NotImplementedError
