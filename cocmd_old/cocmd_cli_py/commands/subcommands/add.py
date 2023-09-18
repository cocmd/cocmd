import click

from cocmd.core.source import Source
from cocmd.utils.repository import find_cocmd_files
from ..groups import add
from cocmd.settings import click_pass_settings
import inquirer
from cocmd.utils.console import console
from cocmd import resources
import os


@add.command()
@click.argument("source")
@click_pass_settings
def source(settings, source: str):
    """
    add a source
    """

    source_label = source

    if source_label == "demo":
        source_label = os.path.join(os.path.dirname(resources.__file__), source_label)

    locations = find_cocmd_files(source_label, settings.scan_depth)

    lst_locs = "\n  - ".join(locations)
    console.print(
        f"""found {len(locations)} cocmd sources in this path:
  - {lst_locs}
    """
    )
    questions = [
        inquirer.Confirm("sure", message="Continue?", default=True),
    ]

    answers = inquirer.prompt(questions)

    if answers["sure"]:
        for loc in locations:
            source = Source(loc, settings)
            settings.sources_manager.add_source(source)
            console.print(f"[bold green]Source '{source}' added")
    else:
        console.print("[bold red]Skipped. you answered 'NO'")
