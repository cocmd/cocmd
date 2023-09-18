from cocmd.settings import click_pass_settings
from cocmd.commands.groups import show
from cocmd.utils.console import console


@show.command()
@click_pass_settings
def sources(settings):
    """
    Show sources
    """
    sources = settings.sources_manager.sources

    for source in sources:
        console.print(f"{source.name}", style="white on blue")
        console.print(f" - {source.location}", style="blue")


@show.command()
@click_pass_settings
def source(settings):
    """
    Show source
    """
