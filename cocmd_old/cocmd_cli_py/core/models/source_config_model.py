from dataclasses import dataclass, field
from cocmd.core.os import OS
from cocmd.utils.io import DictLoader, YamlIO, normalize_path
from typing import List, Optional
from .script_model import ScriptModel


@dataclass
class Automation:
    name: str
    file: Optional[str]
    content: Optional[ScriptModel]

    def load_content(self, location: str):
        if self.file:
            self.content = YamlIO.from_file(
                normalize_path(self.file, location),
                cls=ScriptModel,
            )

    def supports_os(self, os: OS) -> bool:
        return self.content.env in (os, OS.ANY)


@dataclass
class SourceConfigModel(DictLoader):
    name: str
    aliases: Optional[str] = field(default_factory=str)
    paths: Optional[List[str]] = field(default_factory=list)
    automations: Optional[List[Automation]] = field(default_factory=list)
