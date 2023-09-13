from cocmd_cli import resources
import os
from cocmd_cli.core.source import Source
from cocmd_cli.settings import Settings
from cocmd_cli.utils.repository import find_cocmd_files

MD_TECH = """---
sidebar_position: {position}
---

# {name}

## aliases

```bash
{aliases}
```

## paths

{paths}


### automations

{automations}
"""

if __name__ == "__main__":
    demo_path = os.path.join(os.path.dirname(resources.__file__), "demo")
    locations = find_cocmd_files(demo_path, 2)

    settings = Settings()

    for ii, loc in enumerate(locations):
        source = Source(loc, settings)

        paths = "\n".join(map(lambda s: f"- {s}", source.paths))

        automations = "\n".join(
            map(
                lambda a: f"- `{source.name}.{a.name}` - {a.content.description}",
                source.automations,
            )
        )

        with open(f"./website/docs/technologies/{source.name}.md", "w") as fp:
            fp.write(
                MD_TECH.format(
                    name=source.name.capitalize(),
                    position=ii + 1,
                    aliases=source.aliases,
                    paths=paths,
                    automations=automations,
                ),
            )
