default:
    @just -l

clean:
    @cargo clean

check:
    @cargo clippy -- -D warnings

format:
    @cargo fmt

run:
    @cargo run -p extract-drugs -- --algorithm "l" \
        --max-edits 2 \
        --id-column "Case Number" \
        --target-column "Primary Cause" \
        --search-words "coacine|heroin|Fentanyl" \
        --format csv \
        cli/data/Medical_Examiner_Case_Archive.csv

build-prod: format check
    @cargo build --release

test: clean check
    @cargo test

doc: clean check
    @cargo doc --no-deps --open