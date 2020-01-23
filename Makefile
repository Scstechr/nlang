main:
	cargo fmt
	cargo run --release

build:
	cargo fmt
	cargo build --release

test:
	cargo fmt
	cargo test -- --nocapture

