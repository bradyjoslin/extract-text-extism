build:
	cargo build --release --target wasm32-unknown-unknown

test:
	extism call ./target/wasm32-unknown-unknown/release/extract_text_extism.wasm extract --input="<div><p>hi</p>you</div>" --log-level=info