bench:
	@echo "Running rust benchmarks..."
	@echo $(shell date) >> logs/bench_results.log
	@cargo criterion --output-format verbose 2>> logs/bench_results.log
	@echo "==========================================================" >> logs/bench_results.log
	
report:
	@echo "Generating rust report..."
	@open ./target/criterion/reports/Algorithms/index.html