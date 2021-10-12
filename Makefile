build:
	@echo "Building binaries..."
	@./scripts/build.sh
	@echo "Binaries available in ./bin/*"

run-test:
	@go run main.go pipeline "data/pokemon.csv" --id-col "#" --target-col "Name" --clean