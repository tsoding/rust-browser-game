game.wasm: game.rs
	rustc -C opt-level=3 --target wasm32-unknown-unknown game.rs
