import click
from cocmd_cli.settings import click_pass_settings
from cocmd_cli.core.script_runner import ScriptRunner
import inquirer
from cocmd_cli.utils.console import console, error_console


@click.command(
    context_settings=dict(
        ignore_unknown_options=True,
        allow_extra_args=True,
    )
)
@click.argument("name", required=False)
@click.option(
    "-y",
    "--yes",
    is_flag=True,
    default=False,
    help="Don't ask 'are you sure' for every step",
)
@click_pass_settings
@click.pass_context
def run(ctx, settings, name: str, yes: bool):
    """
    Run something
    """

    avilable_automations = settings.sources_manager.automations

    if not name:
        questions = [
            inquirer.List(
                "script",
                message="What script to run?",
                choices=tuple(avilable_automations.keys()),
            ),
        ]

        answers = inquirer.prompt(questions)
        name = answers["script"]

    if name in avilable_automations:
        script = avilable_automations[name]

        script_args = ctx.args
        # print('args=', script_args)
        output = ScriptRunner.run(
            script, settings.os, script_args, settings, auto_yes=yes
        )

        console.print("[blue] Script executed:")
        for line in output:
            console.print(f" - {line}")

        console.print(f"[bold green]Script {script.name} completed")
    else:
        error_console.print("I don't know this script")
