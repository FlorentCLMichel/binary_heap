test:
	cargo test --offline

clippy:
	cargo clippy --offline

clean:
	rm -rf target; rm -f Cargo.lock
