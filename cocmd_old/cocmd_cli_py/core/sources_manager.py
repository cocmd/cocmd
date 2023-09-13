from typing import Sequence
from cocmd_cli.utils.io import file_read_lines, file_write_lines
from cocmd_cli.core.source import Source


class SourcesManager:
    def __init__(self, settings: "Settings"):
        self.settings = settings
        self.sources_file = settings.sources_file
        self.sources = self.load_sources(self.sources_file, settings)

    def remove_source(self, source: Source):
        self.sources.remove(source)
        self.save()

    def add_source(self, source: Source):
        self.sources.add(source)
        self.save()

    def save(self):
        self.save_sources(self.sources_file, self.sources)

    @staticmethod
    def save_sources(sources_file, sources: Sequence[Source]):
        file_write_lines(sources_file, tuple(map(str, sources)))

    @staticmethod
    def load_sources(sources_file, settings):
        return set(map(lambda s: Source(s, settings), file_read_lines(sources_file)))

    @property
    def automations(self):
        automations = {}
        for source in self.sources:
            for automation in source.automations:
                automations[f"{source.name}.{automation.name}"] = automation
        return automations
