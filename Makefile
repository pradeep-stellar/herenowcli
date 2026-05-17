.PHONY: build check clean fmt help lint release run test

CARGO ?= cargo
TARGET ?=
TARGET_FLAG := $(if $(TARGET),--target $(TARGET),)

help:
	@printf '%s\n' \
		'Targets:' \
		'  build   Build the debug binary' \
		'  check   Run format, clippy, and tests' \
		'  clean   Remove Cargo build artifacts' \
		'  fmt     Format Rust source' \
		'  lint    Run clippy with warnings denied' \
		'  release Build the optimized release binary; pass TARGET=<triple>' \
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

release:
	$(CARGO) build --release --locked $(TARGET_FLAG)

run:
	$(CARGO) run -- $(ARGS)

test:
	$(CARGO) test
