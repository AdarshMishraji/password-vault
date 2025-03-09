run-app:
	@echo "Running app"
	RUST_LOG=tower_http=trace cargo run --bin=app

run-migration:
	@echo "Running migration"
	sea-orm migrate