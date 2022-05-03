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
    nargs=2,
)
@click.option(
    "--report/--no-report",
    default=True,
    is_flag=True,
    help="Analyze the data and generate a report",
)
@click.option(
    "--live/--no-live",
    default=True,
    is_flag=True,
    help="Live mode, use our live template.",
)
@click.option(
    "--search-file",
    default=None,
    type=click.Path(exists=True),
    required=False,
    help="Custom search file. Must provide if live is false.",
)
def execute(
    file_name: pathlib.Path,
    id_column: str,
    target_columns: tuple[str, str],
    report: bool,
    live: bool,
    search_file: Optional[str],
):
    print("[cyan]Running wrapper program...")
    utils.run(file_name, id_column, target_columns, report, live, search_file)
    print("[green]Finished wrapper program!")
