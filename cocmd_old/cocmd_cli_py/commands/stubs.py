import click
from cocmd.settings import click_pass_settings


@click.command()
@click_pass_settings
def profile_loader(settings):
    # apply it in bashrc with
    # cd /workspaces/cocmd/ && eval "$(python -m cocmd profile-loader)"

    # apply aliases
    sources = settings.sources_manager.sources

    for source in sources:
        # console.print(f"loading {source.name}", style="white on blue")
        print(source.aliases)

    # apply path
    for p in source.paths:
        print(f'export PATH="{p}:$PATH"')

    for name in settings.sources_manager.automations.keys():
        print(f'alias {name}="cocmd run {name}"')


@click.command()
@click_pass_settings
def refresh(settings):
    settings.sources_manager.save()
