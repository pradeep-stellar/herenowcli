.PHONY: build check clean fmt help lint run test

CARGO ?= cargo

help:
	@printf '%s\n' \
		'Targets:' \
		'  build   Build the debug binary' \
		'  check   Run format, clippy, and tests' \
		'  clean   Remove Cargo build artifacts' \
		'  fmt     Format Rust source' \
		'  lint    Run clippy with warnings denied' \
		'  run     Run the CLI; pass ARGS="--help"' \
		'  test    Run tests'

build:
	$(CARGO) build

check: fmt lint test

clean:
	$(CARGO) clean

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

run:
	$(CARGO) run -- $(ARGS)

test:
	$(CARGO) test
