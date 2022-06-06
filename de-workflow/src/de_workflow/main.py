from typing import Optional
from rich import print
import click

import pathlib
from . import utils

app = click.Group()


@app.command()
@click.argument(
    "file_name",
    type=click.Path(exists=True),
    required=True,
)
@click.argument("id-column", type=click.STRING, required=True)
@click.argument(
    "target-columns",
    type=click.STRING,
    required=True,
    nargs=-1,
)
@click.option(
    "--algorithm",
    default="osa",
    type=click.Choice(["levenshtein", "osa"]),
    required=False,
)
def execute(
    file_name: pathlib.Path,
    id_column: str,
    target_columns: list[str],
    algorithm: str,
):
    print("[cyan]Running wrapper program...")
    utils.run(
        file_name=file_name,
        id_column=id_column,
        target_columns=target_columns,
        algorithm=algorithm,
    )
    print("[green]Finished wrapper program!")
