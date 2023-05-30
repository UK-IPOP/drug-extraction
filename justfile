set shell := ["fish", "-c"]

# list recipies
default:
    @just -l


# for python files/notebooks in examples folder and rust source code
clean-and-lint:
    cd cli && cargo fmt
    cd cli && cargo clippy
    black examples
    isort examples
    ruff examples


# building and publishing is done together 
# to ensure rust/python are always in sync

# test the rust cli
test: clean-and-lint
    cd cli && cargo test


# to build the CLIs in release mode
build: test
    cd cli && cargo build --release
    cd cli && maturin build
    

# install the built python CLI into the currently active python environment
develop: build
    # force reinstall to update without updating version
    cd cli && maturin develop


# publish both CLI targets
publish:
    cd cli && cargo publish
    cd cli && maturin publish
