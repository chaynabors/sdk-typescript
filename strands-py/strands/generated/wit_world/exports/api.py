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
from strands.generated.wit_world.imports import types


class ResponseStream(Protocol):
    @abstractmethod
    def read_next(self) -> Optional[List[types.StreamEvent]]:
        raise NotImplementedError

    @abstractmethod
    def respond(self, args: types.RespondArgs) -> None:
        raise NotImplementedError

    @abstractmethod
    def cancel(self) -> None:
        raise NotImplementedError


class Agent(Protocol):
    @abstractmethod
    def __init__(self, config: types.AgentConfig) -> None:
        raise NotImplementedError

    @abstractmethod
    def generate(self, args: types.StreamArgs) -> ResponseStream:
        raise NotImplementedError

    @abstractmethod
    def get_messages(self) -> str:
        raise NotImplementedError

    @abstractmethod
    def set_messages(self, args: types.SetMessagesArgs) -> None:
        raise NotImplementedError
