import pandas as pd
import datapane as dp
import plotly.express as px
import plotly.io as pio

pio.templates.default = "seaborn"  # type: ignore


def load_data() -> tuple[pd.DataFrame, pd.DataFrame]:
    df1 = pd.read_csv("./dense_results.csv", low_memory=False)
    df2 = pd.read_csv("./merged_results.csv", low_memory=False)
    return df1, df2


page2_md = """
# Data

On this page you will find the actual output data available for exploration.

## Dense Dataframe

This comes from the [drug extraction tool](https://github.com/UK-IPOP/drug-extraction) (with some additions).

This table is useful for counts and other analyses that are not specific to the record.

{{table1}}

## Merged Dataframe

This table transforms the dense dataframe into a more useful format (wide-form/record-wise) 
and then merges it back to the original dataframe.

This table is useful for analyses that utilize data from the original table.

{{table2}}
"""

page1_md = """
This page contains information and quick analysis about the detected drugs. Brought to you by 
[UK-IPOP](https://pharmacy.uky.edu/office-research-operations/cornerstones/research-centers/ipop) 
[GitHub](https://github.com/UK-IPOP).

## KPIs

{{kpis}}

## Drug Counts

{{drug_counts_group}}

## Records and Levels

{{records_group}}
"""

header_section = dp.Group(
    dp.Media("./images/IPOP-logo.png"),
    dp.Media("./images/COP-logo.png"),
    columns=2,
)


def make_kpis(data: pd.DataFrame) -> dp.Group:
    # expects dense data
    drug_counts = data.search_term.value_counts().reset_index()
    drug_counts.columns = ["search_term", "counts"]
    drug_counts = drug_counts.iloc[0]
    drug_value = f"{drug_counts.search_term} ({drug_counts.counts})"

    record_counts = data.record_id.value_counts().reset_index()
    record_counts.columns = ["record_id", "counts"]
    record_counts = record_counts.iloc[0]
    record_value = f"{record_counts.record_id} ({record_counts.counts})"

    kpis = dp.Group(
        dp.BigNumber(value=drug_value, heading="Most common search term:"),
        dp.BigNumber(value=record_value, heading="Record ID with most drugs:"),
        columns=2,
    )
    return kpis


def search_terms_counts_plot(data: pd.DataFrame) -> dp.Plot:
    # expects to get dense data
    dfg = (
        data.groupby("search_term")
        .size()
        .sort_values(ascending=False)
        .head(10)
        .reset_index()
    )
    dfg.columns = ["search_term", "term_count"]
    fig = px.histogram(
        dfg, x="search_term", y="term_count", title="Top 10 Search Words"
    )
    fig.update_layout(
        xaxis_title="Search Term",
        yaxis_title="Count",
    )
    return dp.Plot(fig)


def tag_counts_plot(data: pd.DataFrame) -> dp.Plot:
    # expects to get dense data
    tags = [x for v in data.tags.values for x in v.split(";")]
    tags = pd.Series(tags, name="tags")
    tag_counts = tags.value_counts().head(10).reset_index()
    tag_counts.columns = ["tag", "counts"]
    fig = px.histogram(tag_counts, x="tag", y="counts", title="Top 10 Tags")
    fig.update_layout(
        xaxis_title="Tag",
        yaxis_title="Count",
    )
    return dp.Plot(fig)


def record_counts_plot(data: pd.DataFrame) -> dp.Plot:
    # expects to get dense data
    record_counts = data.record_id.value_counts().head(10).reset_index()
    record_counts.columns = ["record_id", "counts"]
    fig = px.histogram(record_counts, x="record_id", y="counts", title="Top 10 Records")
    fig.update_layout(
        xaxis_title="Record ID",
        yaxis_title="# of Drugs Detected",
    )
    return dp.Plot(fig)


def primary_vs_secondary_plot(data: pd.DataFrame) -> dp.Plot:
    # expects to get dense data
    counts = data.source_column.value_counts().reset_index()
    counts.columns = ["source_columns", "counts"]
    fig = px.bar(
        counts, x="source_columns", y="counts", title="Primary vs Secondary Detections"
    )
    fig.update_layout(
        xaxis_title="Source Column",
        yaxis_title="Count",
    )
    return dp.Plot(fig)


def make_tab1(dense: pd.DataFrame) -> dp.Text:
    tab = dp.Text(page1_md, label="Report").format(
        kpis=make_kpis(dense),
        drug_counts_group=dp.Group(
            search_terms_counts_plot(dense),
            tag_counts_plot(dense),
            columns=2,
        ),
        records_group=dp.Group(
            record_counts_plot(dense),
            primary_vs_secondary_plot(dense),
            columns=2,
        ),
    )
    return tab


def make_tab2(dense: pd.DataFrame, merged: pd.DataFrame) -> dp.Text:
    tab = dp.Text(page2_md, label="Data").format(
        table1=dp.DataTable(dense),
        table2=merged,
    )
    return tab


def make_report() -> dp.Report:
    dense, merged = load_data()

    report = dp.Report(
        header_section,
        dp.Select(
            blocks=[
                make_tab1(dense),
                make_tab2(dense, merged),
            ]
        ),
    )
    return report


def generate_report():
    report = make_report()
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


if __name__ == "__main__":
    generate_report()
