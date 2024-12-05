.PHONY: release
release:
	cargo build --release


.PHONY: run
run:
	cargo build --release && ./target/release/invaders