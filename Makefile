
target/debug/yard-lang: src/main.rs
	cargo build

.PHONY:	test

test:target/debug/yard-lang
	cargo test
