# Linting.
lint:
	cargo clippy --all-targets --all-features -- -D warnings \
		--allow clippy::needless-lifetimes
	cargo sort --check --check-format

# Run the unit tests.
unit_test:
	cargo test

# Standard build target.
build:
	cargo build

# Run the compiled program.
run:
	cargo run

# Command to run before commiting.
pre_commit: lint unit_test

# Define the targets that do not create files.
.PHONY: lint unit_test build run pre_commit