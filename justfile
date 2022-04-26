default:
    @just -l

clean:
    @cargo clean

check:
    @cargo clippy -- -D warnings

format:
    @cargo fmt

run:
    @cargo run -p drug-extraction-cli -- simple-search \
        cli/data/records.csv \
        --algorithm "l" \
        --max-edits 1 \
        --id-column "Case Number" \
        --target-column "Primary Cause" \
        --search-words "coacine|heroin|Fentanil" \
        --format csv \
        --analyze 

build-prod: format check
    @cargo build --release

test: clean check
    @cargo test

doc: clean check
    @cargo doc --no-deps --open