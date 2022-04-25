default:
    @just -l

clean:
    @cargo clean

check:
    @cargo clippy -- -D warnings

format:
    @cargo fmt

run:
    @cargo run -p extract-drugs -- run \
        --algorithm "l" \
        --max-edits 2 \
        --id-column "Case Number" \
        --target-column "Primary Cause" \
        --search-words "coacine|heroin|Fentanyl" \
        --format csv \
        --rx-class-id "N02A" \
        --rx-class-source "ATC" \
        cli/data/records.csv

build-prod: format check
    @cargo build --release

test: clean check
    @cargo test

doc: clean check
    @cargo doc --no-deps --open