build-all:
	cargo build --release
	cargo build --target wasm32-unknown-unknown --release

clean:
	cargo clean
