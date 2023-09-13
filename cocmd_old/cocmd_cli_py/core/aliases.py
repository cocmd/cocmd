


import itertools
from typing import Sequence
from cocmd_cli.core.source import Source
from cocmd_cli.utils.io import file_write_lines


class Aliases:
    @staticmethod
    def recreate_aliases(dest_aliases:str, sources: Sequence[Source]):

        def _assert_alias_line(s):
            if not s:
                return s
            if not s.startswith('alias '):
                return f'alias {s}'
            return s

        file_write_lines(dest_aliases, itertools.chain.from_iterable(
            map(_assert_alias_line, source.aliases)
            for source in sources
        ))