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


def call_tool(args: types.CallToolArgs) -> str:
    """Raises: `Err[str]`"""
    raise NotImplementedError


def call_tools(args: types.CallToolsArgs) -> List[Result[str, str]]:
    raise NotImplementedError
