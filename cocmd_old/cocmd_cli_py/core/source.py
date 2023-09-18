import os
from cocmd.consts import Consts
from cocmd.core.models.source_config_model import SourceConfigModel
from cocmd.utils.io import YamlIO, exists, normalize_path


class Source:
    def __init__(self, _location: str, settings: "Settings"):
        self.settings = settings
        self._location = _location.lower()

        if exists(self._location):
            self._location = os.path.abspath(self._location)  # convert to abs path
            self._cocmd_config = YamlIO.from_file(
                os.path.join(self._location, Consts.SOURCE_CONFIG_FILE),
                cls=SourceConfigModel,
            )

        # elif self._location.endswith(".git"):
        #     local_repo = os.path.join(settings.home, git.get_repo_name(self._location))
        #     if not exists(local_repo):
        #         git.clone(self._location, local_repo)

        #     self._cocmd_config = YamlIO.from_file(
        #         os.path.join(local_repo, Consts.SOURCE_CONFIG_FILE),
        #         cls=SourceConfigModel,
        #     )
        else:
            raise RuntimeError(
                f"path {self._location} not exists. edit `~/.cocmd/sources.txt` and remove it manually"
            )

    @property
    def is_exists_locally(self):
        return False

    @property
    def aliases(self):
        return self._cocmd_config.aliases

    @property
    def name(self):
        return self._cocmd_config.name

    @property
    def paths(self):
        return tuple(
            normalize_path(p, self._location) for p in self._cocmd_config.paths
        )

    @property
    def automations(self):
        for automation in self._cocmd_config.automations:
            automation.load_content(self._location)
            if automation.supports_os(self.settings.os):
                yield automation

    @property
    def location(self):
        return self._location

    def __repr__(self):
        return str(self._location)

    def __str__(self):
        return str(self._location)

    def __eq__(self, other):
        return str(self) == str(other)

    def __hash__(self):
        return hash((type(self), self._location))
