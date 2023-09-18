from os import path
from typing import Sequence, List
import webbrowser
import inquirer
from cocmd.consts import Consts
from cocmd.core.models.script_model import ScriptModel, StepRunnerType
from rich.markdown import Markdown

from cocmd.core.os import OS
from cocmd.utils import io
import subprocess
from cocmd.utils.console import console, error_console
from collections import OrderedDict


class ScriptRunner:
    @staticmethod
    def run(
        script: ScriptModel,
        os: OS,
        script_args: Sequence[str],
        settings: "Settings",
        auto_yes: bool = False,
    ) -> List[str]:
        def out(command, *script_args, runner: StepRunnerType):
            exec_file = path.join(io.get_tmp(), Consts.TMP_EXEC_FILE_NAME)
            io.file_write(exec_file, command)

            if runner == StepRunnerType.PYTHON:
                p = subprocess.run(
                    "python " + exec_file + " " + " ".join(script_args), shell=True
                )
            else:
                io.chmod_x(exec_file)
                p = subprocess.run(exec_file + " " + " ".join(script_args), shell=True)
            return p

        console.print(script.name, style="frame white on blue")

        steps_choices = OrderedDict()
        for ii, step in enumerate(script.content.steps):
            steps_choices[f"{ii} - {step.title}"] = step

        questions = [
            inquirer.Checkbox(
                "steps",
                message="Select what to execute: (all by default)",
                choices=list(steps_choices.keys()),
                default=list(steps_choices.keys()),
            ),
        ]

        answers = inquirer.prompt(questions)

        chosen_steps = tuple(
            step for label, step in steps_choices.items() if label in answers["steps"]
        )
        chosen_steps = tuple(steps_choices.values())

        # console.print(f'Executing {len(chosen_steps)} steps:')

        output = []
        for ii, step in enumerate(chosen_steps):
            res = "ok"
            if step.runner == StepRunnerType.SHELL:
                if ScriptRunner.ask_if_ok_to_execute(
                    f"step {ii} - {step.title}", "Execute?", auto_yes
                ):
                    r = out("set -x\n" + step.content, *script_args, runner=step.runner)

                    if r.returncode:
                        error_console.print("failed to run step")
                        res = "failed"
                else:
                    res = "skipped"
            elif step.runner == StepRunnerType.PYTHON:
                if ScriptRunner.ask_if_ok_to_execute(
                    f"step {ii} - {step.title}", "Execute?", auto_yes
                ):
                    r = out(step.content, *script_args, runner=step.runner)

                    if r.returncode:
                        error_console.print("failed to run step")
                        res = "failed"
                else:
                    res = "skipped"
            elif step.runner == StepRunnerType.MARKDOWN:
                markdown = Markdown(step.content)
                console.print(markdown)
            elif step.runner == StepRunnerType.LINK:
                if ScriptRunner.ask_if_ok_to_execute(
                    f"step {ii} - {step.title} -> link to {step.content}",
                    "Open Link?",
                    auto_yes,
                ):
                    webbrowser.open(step.content, new=2)
                    console.print()
                else:
                    res = "skipped"

            elif step.runner == StepRunnerType.COCMD_SCRIPT:
                nested_script = settings.sources_manager.scripts[step.content]
                output.extend(ScriptRunner.run(nested_script, os, [], settings))
            else:
                raise NotImplementedError()

            if step.runner != StepRunnerType.COCMD_SCRIPT:
                if res == "skipped":
                    output.append(f'skipped [strike]"{step.title}"')
                elif res == "failed":
                    output.append(f'[red] x "{step.title}"')
                else:
                    output.append(f'[green] ✓ "{step.title}"')

        return output

    # @staticmethod
    # def iterate_steps(script: ScriptModel, variation: StepsModel, settings: "Settings"):
    #     for step in variation.steps:
    #         if isinstance(step, StepRefModel):
    #             try:
    #                 step = next(
    #                     global_step
    #                     for global_step in script.spec.globals
    #                     if global_step.id == step.ref
    #                 )
    #             except StopIteration:
    #                 # no results from globals
    #                 try:
    #                     ref_script = settings.sources_manager.scripts[step.ref]
    #                     step = StepModel(
    #                         title=ref_script.title,
    #                         description=ref_script.description,
    #                         runner=StepRunnerType.COCMD_SCRIPT,
    #                         content=step.ref,
    #                     )
    #                 except KeyError:
    #                     # can't find the script in cocmd or script globals
    #                     raise ValueError(f"unable to find reference {step.ref}")

    #         yield step

    @staticmethod
    def ask_if_ok_to_execute(text: str, question: str, auto_yes: bool) -> bool:
        if auto_yes:
            return True

        console.print(text, style="blue")
        questions = [inquirer.Confirm("sure", message=question, default=True)]

        answers = inquirer.prompt(questions)

        return answers["sure"]
