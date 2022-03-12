bench:
	@echo "Running rust benchmarks..."
	@echo $(shell date) >> logs/results.log
	@cargo criterion --output-format verbose 2>> logs/results.log
	@echo "==========================================================" >> logs/results.log
	
report:
	@echo "Generating rust report..."
	@open ./target/criterion/reports/Algorithms/index.html