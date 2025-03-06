run-app:
	@echo "Running app"
	cargo run --bin=app

run-migration:
	@echo "Running migration"
	sea-orm migrate