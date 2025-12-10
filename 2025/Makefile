default: run_dev

clean:
	@echo "Cleaning using cargo..."
	@cargo clean

check:
	@echo "Checking..."
	@cargo check

clippy:
	@echo "Running Clippy..."
	@cargo clippy -- -W clippy::pedantic

clippy_fix:
	@echo "Running Clippy fixes..."
	@cargo clippy --fix -- -W clippy::pedantic

test:
	@echo "Running tests..."
	@cargo test

build_dev:
	@echo "Building debug..."
	@cargo build --features env_file

build_release:
	@echo "Building release..."
	@cargo build --release

run_dev:
	@echo "Running debug..."
	@cargo run --features env_file

run_release:
	@echo "Running release..."
	@cargo run --release
