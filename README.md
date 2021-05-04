# Bloat-Free Browser Game in Rust (rustc-only challenge)

*Don't blame me if this code breaks in 1 years*

The idea is to make as bloat-free game in Rust as possible (not using [Emscripten](https://emscripten.org/), [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), etc.) using only `rustc` executable.

## Quick Start

It is assumed that you are using [rustup](https://rustup.rs/) for managing your local Rust installation.

1. Make sure you have `wasm32-unknown-unknown` target installed:

```console
$ rustup install target wasm32-unknown-unknown
```

2. Build the game

```console
$ make
```

3. Play the game

```console
$ iexplore index.html
```
