all: test

t:
	$(MAKE) test

test:
	cargo nextest run
	cargo fmt

# Running benchmarks requires nightly Rust
bench:
	cargo +nightly bench

# Install
i:
	cargo install --locked cargo-nextest

.PHONY: f bench i
