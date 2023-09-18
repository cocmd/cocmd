from dataclasses import dataclass, field
from cocmd.utils.io import DictLoader
from typing import Optional


@dataclass
class CredsConfigModel(DictLoader):
    token: Optional[str] = field(default=None)

    @property
    def is_valid(self):
        return bool(self.token)
