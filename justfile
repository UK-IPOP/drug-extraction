default:
    @just -l

clean:
    @cargo clean

check:
    @cargo clippy -- -D warnings

format:
    @cargo fmt

run:
    @cargo run -p extract-drugs --  cli/data/Medical_Examiner_Case_Archive.csv "cocaine|fentanyl|heroin" 0.95

build-prod: format check
    @cargo build --release

test: clean check
    @cargo test

doc: clean check
    @cargo doc --no-deps --open