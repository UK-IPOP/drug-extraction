bench:
	@echo "Running go benchmarks..."
	@go test -bench=. --benchtime 100000x


bench-save:
	@echo "Running go benchmarks..."
	@echo $(shell date) >> logs/bench_results.log
	@go test -bench=. --benchtime 100000x >> logs/bench_results.log
	@echo "==========================" >> logs/bench_results.log


test:
	@echo "Running go tests..."
	@go test
