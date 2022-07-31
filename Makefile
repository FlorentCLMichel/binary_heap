test:
	cargo test --offline

clippy:
	cargo clippy --offline

sort_by_key: 
	cargo run --offline --example sort_by_key

clean:
	rm -rf target; rm -f Cargo.lock
