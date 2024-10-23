# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version            # rust compiler
	cargo --version            # rust package manager
	rustfmt --version          # rust code formatter
	rustup --version           # rust toolchain manager
	clippy-driver --version    # rust linter

# Format Rust code
format:
	cargo fmt --quiet

# Lint Rust code
lint:
	cargo clippy --quiet

# Run Rust tests
test:
	cargo test --quiet

# Build the Rust project
build:
	cargo build

# Build and run the Rust binary
run: build
	cargo run

# Build optimized binary
release:
	cargo build --release

# Perform all tasks: format, lint, build, test, and run
all: format lint build test run
