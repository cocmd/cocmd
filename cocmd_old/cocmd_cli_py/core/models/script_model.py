from dataclasses import dataclass, field
from cocmd.core.os import OS
from cocmd.utils.io import DictLoader
from typing import List, Optional
from enum import Enum


class StepRunnerType(Enum):
    SHELL = "shell"
    MARKDOWN = "markdown"
    PYTHON = "python"
    COCMD_SCRIPT = "cocmd_script"
    LINK = "link"


@dataclass
class StepModel(DictLoader):
    runner: StepRunnerType
    content: str
    title: str


@dataclass
class ScriptModel(DictLoader):
    steps: List[StepModel]
    env: Optional[OS] = field(default=OS.ANY)
    description: Optional[str] = field(default=None)
