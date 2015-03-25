all: dependencies test

test:
	cargo test

dependencies:
	which cargo

.PHONY: all dependencies test
