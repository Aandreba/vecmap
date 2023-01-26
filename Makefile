doc:
	cargo +nightly rustdoc --open --all-features -- --cfg docsrs

check:
	cargo check
	cargo +nightly check --features alloc