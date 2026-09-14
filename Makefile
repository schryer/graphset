# Single entry point, mirroring the publet workspace this crate was
# extracted from.
SHELL := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
export PATH := $(HOME)/.cargo/bin:$(PATH)

.PHONY: check fmt-check lint guard-floats test build clean

check: fmt-check lint guard-floats test ## Everything CI should run

fmt-check: ## Formatting, unchanged
	cargo fmt --all -- --check

lint: ## Clippy, pedantic
	cargo clippy --all-targets -- -D warnings

guard-floats: ## No f32/f64 anywhere in src/
	./tools/check-floats.sh

test: ## Unit, integration, and doc tests
	cargo test

build: ## Debug build
	cargo build

clean: ## Remove build artifacts
	cargo clean
