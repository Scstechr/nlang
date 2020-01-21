main:
	cargo fmt
	cargo build --release

test:
	cargo test -- --nocapture
