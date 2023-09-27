from enum import Enum
from dacite import from_dict, Config
from dataclasses import asdict, dataclass
import yaml
import stat

import tempfile
import os
from pathlib import Path
from typing import Sequence
from cocmd.core.os import OS


def normalize_path(relative_path: str, base_path: str = None) -> str:
    if base_path:
        # Join the base_path with the relative_path and then resolve to an absolute path
        return os.path.abspath(os.path.join(base_path, relative_path))
    # Resolve the relative_path to an absolute path using the current working directory
    return os.path.abspath(relative_path)


def exists(path) -> bool:
    return os.path.exists(path)


def mkdir(dir):
    Path(dir).mkdir(parents=True, exist_ok=True)


def touch(file):
    Path(file).touch()


def file_read_lines(file) -> Sequence[str]:
    with open(file, "r") as fp:
        return map(lambda s: s.strip(), fp.readlines())


def file_write_lines(file, lines):
    with open(file, "w") as fp:
        fp.writelines(map(lambda s: f"{s}\n", lines))


def file_write(file, content):
    with open(file, "w") as fp:
        fp.write(content)


def get_tmp_file():
    return tempfile.NamedTemporaryFile()


def get_tmp():
    return tempfile.gettempdir()


def chmod_x(file):
    st = os.stat(file)
    os.chmod(file, st.st_mode | stat.S_IEXEC)


@dataclass
class DictLoader:
    @classmethod
    def from_dict(cls, data):
        return from_dict(
            data_class=cls,
            data=data,
            config=Config(cast=[Enum], type_hooks={OS: OS.from_str}),
        )

    def to_dict(self):
        def _dict_factory(data):
            return {
                field: value.value if isinstance(value, Enum) else value
                for field, value in data
            }

        return asdict(self, dict_factory=_dict_factory)


class YamlIO:
    @staticmethod
    def from_file(file: str, cls: DictLoader = None) -> DictLoader:
        with open(file, "r") as fp:
            text = fp.read()
            obj = yaml.safe_load(text)

        if cls:
            obj = cls.from_dict(obj)
        return obj

    @staticmethod
    def to_file(file: str, data: DictLoader):
        with open(file, "w") as fp:
            yaml.dump(data.to_dict(), fp, default_flow_style=False)
