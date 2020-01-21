main:
	cargo fmt
	cargo build --release

test:
	cargo fmt
	cargo test -- --nocapture
