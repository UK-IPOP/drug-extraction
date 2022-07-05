bench-save:
	@echo "Running python benchmarks..."
	@echo $(shell date) >> logs/bench_results.log
	@poetry install
	@poetry run python -m pytest --color=auto --benchmark-histogram="logs/bench_hist" >> logs/bench_results.log
	@open logs/bench_hist.svg


bench:
	@echo "Running python benchmarks..."
	@poetry install
	@poetry run python -m pytest --color=auto --benchmark-histogram="logs/bench_hist"
	@open logs/bench_hist.svg

test:
	@echo "Running python tests..."
	@poetry run python -m pytest --color=auto