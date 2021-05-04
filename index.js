async function start() {
    const memory = new WebAssembly.Memory({initial: 300, maximum: 1000});
    const memoryView = new Uint8Array(memory.buffer);

    const game = await WebAssembly.instantiateStreaming(
        fetch("game.wasm"),
        {
            "env": {
                "memory": memory,
                "imported_func": (x) => {
                    console.log(`Rust says ${x}`);
                    return 69;
                }
            }
        });

    game.instance.exports.next_frame(69.0);
}

start().catch((e) => console.log(e));
