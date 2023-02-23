import collections
import itertools
import numpy as np
import pandas as pd
import datapane as dp
import plotly.express as px
import plotly.io as pio
import polars as pl
from itertools import combinations

pio.templates.default = "seaborn"  # type: ignore


standard_markdown = """
# UK-IPOP Drug Extraction Tool Work Flow Report

This page contains information and quick analysis about the detected terms. Brought to you by 
[UK-IPOP](https://pharmacy.uky.edu/office-research-operations/cornerstones/research-centers/ipop) 
-- [GitHub](https://github.com/UK-IPOP).

## Similarity Score Plot
{{sim_score_plot}}

## Terms Plots
{{terms_plots}}
"""

identified_markdown = """
# UK-IPOP Drug Extraction Tool Work Flow Report

This page contains information and quick analysis about the detected terms. Brought to you by 
[UK-IPOP](https://pharmacy.uky.edu/office-research-operations/cornerstones/research-centers/ipop) 
-- [GitHub](https://github.com/UK-IPOP).

## Similarity Score Plot
{{sim_score_plot}}

## Terms Plots
{{terms_plots}}

## Identified Plots
{{id_plots}}

## Metadata Plot
{{metadata_block}}

## Term Combos
{{term_combos}}
"""


def find_top10(df: pl.LazyFrame, col: str) -> pl.DataFrame:
    return (
        df.select(pl.col(col).value_counts(sort=True))
        .limit(10)
        .unnest(col)
        .rename({"counts": "Count"})
        .collect()
    )


# expects top 10 df
def plot_top_10(
    df: pl.DataFrame,
    col: str,
) -> dp.Plot:
    fig = (
        px.bar(df.to_pandas(), x=col, y="Count")
        .update_yaxes(categoryorder="total ascending")
        .update_layout(
            title=f"Top 10 {col}",
        )
    )
    return dp.Plot(fig)


def plot_sim_score(df: pl.LazyFrame) -> dp.Plot:
    avg_score = df.select("Similarity Score").mean().collect().to_numpy()[0][0]
    fig = px.histogram(
        df.collect().to_pandas(),
        x="Similarity Score",
        nbins=10,
        marginal="rug",
        hover_data=df.columns,
    ).update_layout(
        title=f"Similarity Score Distribution (Mean: {avg_score:.2f})",
        xaxis_title="Similarity Score",
        yaxis_title="Count",
    )
    return dp.Plot(fig)


def make_identified_plots(df: pl.LazyFrame) -> list[dp.Group]:
    cols = [
        "Row ID",
        "Similarity Score",
        "Search Term",
        "Matched Term",
        "Source Column",
    ]
    df = df.select(cols)
    plots = []
    for col in cols:
        if col == "Similarity Score":
            plot = plot_sim_score(df)
        else:
            top_10 = find_top10(df, col)
            plot = plot_top_10(top_10, col)
        plots.append(plot)

    return [
        dp.Group(blocks=[plots[2]], columns=1),
        dp.Group(blocks=[plots[0], plots[4]], columns=2),
        dp.Group(blocks=plots[2:4], columns=2),
    ]


def make_standard_plots(df: pl.LazyFrame) -> list[dp.Group]:
    cols = [
        "Similarity Score",
        "Search Term",
        "Matched Term",
    ]
    df = df.select(cols)
    plots = []
    for col in cols:
        if col == "Similarity Score":
            plot = plot_sim_score(df)
            plots.append(plot)
        else:
            top_10 = find_top10(df, col)
            plot = plot_top_10(top_10, col)
            plots.append(plot)

    return [
        dp.Group(blocks=[plots[0]], columns=1),
        dp.Group(blocks=plots[1:3], columns=2),
    ]


def make_metadata_plot(df: pl.LazyFrame) -> dp.Group:
    metadata_counts = (
        df.select(pl.col("Metadata").str.split("|"))
        .explode("Metadata")
        .select(pl.col("Metadata").value_counts(sort=True))
        .unnest("Metadata")
        .rename({"counts": "Count"})
        .collect()
    )
    fig = px.bar(
        metadata_counts.to_pandas(),
        x="Metadata",
        y="Count",
        hover_data=metadata_counts.columns,
    ).update_layout(
        title="Metadata Distribution",
        xaxis_title="Metadata",
        yaxis_title="Count",
    )
    return dp.Group(blocks=[dp.Plot(fig)], columns=1)


def yield_combinations(items: list[str]):
    for i in range(1, len(items) + 1):
        yield from combinations(items, i)


def combine_items(items: list[str]) -> str:
    if len(items) == 1:
        return items[0]
    else:
        return " <-> ".join(items)


def make_term_combos_plot(df: pl.LazyFrame) -> dp.Group:
    combos = (
        df.select(["Row ID", "Search Term"])
        .groupby("Row ID")
        .agg(pl.col("Search Term").unique().sort().alias("Search Terms"))
        .select(
            pl.col("Search Terms")
            .apply(yield_combinations)
            .apply(list)
            .arr.explode()
            .alias("Search Term Combos")
        )
        .select(
            pl.col("Search Term Combos").apply(combine_items).value_counts(sort=True)
        )
        .unnest("Search Term Combos")
        .rename({"counts": "Count"})
        .with_columns(
            [
                pl.col("Search Term Combos")
                .str.split(" <-> ")
                .arr.lengths()
                .alias("Terms In Combo"),
            ]
        )
        .limit(20)
        .collect()
    )
    fig = px.bar(
        combos.to_pandas(),
        x="Count",
        y="Search Term Combos",
        color="Terms In Combo",
        orientation="h",
        hover_data=combos.columns,
    ).update_layout(
        title="Term Combinations",
    )
    return dp.Group(blocks=[dp.Plot(fig)], columns=1)


def make_report(df: pl.LazyFrame, identified: bool, disable_metadata: bool) -> dp.App:
    logos = dp.Group(
        blocks=[
            dp.Media(file="images/COP-logo.png"),
            dp.Media(file="images/IPOP-logo.png"),
        ],
        columns=2,
    )
    blocks = []
    blocks.append(logos)
    if identified:
        plots = make_identified_plots(df)
        content = dp.Text(identified_markdown, label="Report").format(
            sim_score_plot=plots[0],
            id_plots=plots[1],
            terms_plots=plots[2],
            metadata_block=dp.Group(blocks=[], columns=1)
            if disable_metadata
            else make_metadata_plot(df),
            term_combos=make_term_combos_plot(df),
        )
        blocks.append(content)
        return dp.App(blocks=blocks)

    else:
        plots = make_standard_plots(df)
        content = dp.Text(standard_markdown, label="Report").format(
            sim_score_plot=plots[0],
            terms_plots=plots[1],
        )
        blocks.append(content)
        return dp.App(blocks=blocks)


def generate_report(df: pl.LazyFrame, identified: bool, disable_metadata: bool = False):
    report = make_report(df, identified, disable_metadata)
    report.save(
        "report.html",
        formatting=dp.ReportFormatting(
            text_alignment=dp.TextAlignment.LEFT,
            width=dp.ReportWidth.MEDIUM,
            accent_color="Blue",
            bg_color="Gainsboro",
        ),
        open=True,
    )
