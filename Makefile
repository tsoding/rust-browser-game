game.wasm: game.rs
	rustc -C opt-level=1 --target wasm32-unknown-unknown game.rs
