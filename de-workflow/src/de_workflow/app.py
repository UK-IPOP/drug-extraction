import argparse
import enum
import sys
from pathlib import Path

from rich import print
import polars as pl

from de_workflow.report import generate_report

version = "0.3.0"


def greet() -> None:
    print(
        f"""
[bold]UK-IPOP Drug Extraction Tool WorkFlow[/bold]
[italic]Version {version}[/italic]
This is a tool to generate a report of extracted data from the main CLI.
For more information, please visit the GitHub repository:
[blue]https://github.com/UK-IPOP/drug-extraction[/blue] or the `--help` menu.
"""
    )


def find_data_file() -> Path:
    print("Searching for output file...")
    files = [
        f
        for f in Path.cwd().iterdir()
        if f.is_file() and f.name == "output.csv" or f.name == "output.jsonl"
    ]
    if len(files) == 0:
        print("[red]No output files found, please run the extraction cli first[/red]")
        sys.exit(1)
    elif len(files) > 1:
        print(
            "[yellow]Multiple output files found, please limit to one or provide the `--file` CLI argument.[/yellow]"
        )
        sys.exit(1)
    else:
        print(f"[green]Found output file: {files[0].name}[/green]")
        return files[0]


class OutputType(enum.Enum):
    Standard = 0
    Identified = 1


def identify_output_format(cols: list[str]) -> OutputType:
    standard_cols = ["Similarity Score", "Search Term", "Matched Term", "Metadata"]
    identified_cols = [
        "Row ID",
        "Similarity Score",
        "Search Term",
        "Matched Term",
        "Metadata",
        "Source Column",
    ]
    if cols == standard_cols:
        return OutputType.Standard
    elif cols == identified_cols:
        return OutputType.Identified
    else:
        print(
            "[red]Unknown output file format, please use output from the main CLI[/red]"
        )
        sys.exit(1)


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="de-workflow",
        description="UK-IPOP Drug Extraction Tool WorkFlow",
        epilog="Use this in conjunction with the main CLI to generate a nice report of extracted data. It will generate an interactive HTML report to view the results of the extraction.",
    )
    parser.add_argument(
        "-f",
        "--file",
        nargs=1,
        type=Path,
        required=False,
        help="Override the output file to use. Program effectiveness not guaranteed.",
    )
    parser.add_argument(
        "-n",
        "--no-metadata",
        type=bool,
        required=False,
        help="Do not include metadata in the report. This will ignore the metadata column from the output file.",
    )
    args = parser.parse_args()
    greet()
    if args.file:
        print(
            f"[yellow]Using output file [bold]override[/bold]: {args.file[0]}[/yellow]"
        )
        output_file = Path(args.file[0])
    else:
        output_file = find_data_file()

    if output_file.suffix == ".csv":
        lazy_df: pl.LazyFrame = pl.scan_csv(output_file, low_memory=True)
    elif output_file.suffix == ".jsonl":
        lazy_df: pl.LazyFrame = pl.scan_ndjson(output_file, low_memory=True)
    else:
        print("[red]Unknown file type, please use either .csv or .jsonl[/red]")
        sys.exit(1)

    file_format = identify_output_format(cols=lazy_df.columns)
    disable_metadata = True if args.no_metadata else False
    match file_format:
        case OutputType.Standard:
            generate_report(
                lazy_df, identified=False, disable_metadata=disable_metadata
            )
            print("[green]Done.[/green]")
        case OutputType.Identified:
            generate_report(lazy_df, identified=True, disable_metadata=disable_metadata)
            print("[green]Done.[/green]")
