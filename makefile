.PHONY: run

run:
	@export USER=test; \
	export PASSWORD=test; \
	export HOST=127.0.0.1; \
	export PORT=3306; \
	export DATABASE=test; \
	export RUST_LOG=debug; \
	cargo run