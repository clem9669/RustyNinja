# Makefile for RustyNinja

# Define the project name
PROJECT_NAME = rustyninja

# Define the cargo command
CARGO = cargo

# Define the default target
.DEFAULT_GOAL = build-release-windows-x64

# Help target to display available commands
help:
	@echo "Available commands:"
	@echo "  make build-release-linux       - Build the project in release mode for Linux"
	@echo "  make build-release-windows-x86 - Build the project in release mode for Windows x86"
	@echo "  make build-release-windows-x64 - Build the project in release mode for Windows x64"
	@echo "  make build-all-release         - Build the project in release mode for Linux, Windows x86, and Windows x64"
	@echo "  make test                      - Run the tests"
	@echo "  make clean                     - Clean the project"
	@echo "  make doc                       - Generate documentation"

# Build the project in release mode for Linux
build-release-linux:
	$(CARGO) build --release --target x86_64-unknown-linux-gnu

# Build the project in release mode for Windows x86
build-release-windows-x86:
	$(CARGO) build --release --target i686-pc-windows-gnu

# Build the project in release mode for Windows x64
build-release-windows-x64:
	$(CARGO) build --release --target x86_64-pc-windows-gnu

# Build the project in release mode for Linux, Windows x86, and Windows x64
build-all-release: build-release-linux build-release-windows-x86 build-release-windows-x64

# Run the tests
test:
	$(CARGO) test

# Clean the project
clean:
	$(CARGO) clean

# Generate documentation
doc:
	$(CARGO) doc --open

# Phony targets to avoid conflicts with files of the same name
.PHONY: help build-release-linux build-release-windows-x86 build-release-windows-x64 build-all-release test clean doc
