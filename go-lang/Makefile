bench:
	@echo "Running go benchmarks..."
	@go test -bench=. --benchtime 1000000x


bench-save:
	@echo "Running go benchmarks..."
	@echo $(shell date) >> logs/results.log
	@go test -bench=. --benchtime 1000000x >> logs/results.log
	@echo "==========================" >> logs/results.log