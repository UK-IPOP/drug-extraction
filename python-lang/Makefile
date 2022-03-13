bench-save:
	@echo "Running python benchmarks..."
	@echo $(shell date) >> logs/bench_results.log
	@poetry run python -m pytest --color=auto --benchmark-histogram="logs/bench_hist" >> logs/bench_results.log


bench:
	@echo "Running python benchmarks..."
	@poetry run pytest --color=auto

test:
	@echo "Running python tests..."
	@poetry run pytest --color=auto