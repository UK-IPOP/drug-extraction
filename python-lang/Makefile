bench-save:
	@echo "Running python benchmarks..."
	@echo $(shell date) >> logs/results.log
	@poetry run python -m pytest --color=auto --benchmark-histogram="logs/bench_hist" >> logs/results.log


bench:
	@echo "Running python benchmarks..."
	@poetry run pytest --color=auto