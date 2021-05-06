async function start() {
    const game = await WebAssembly.instantiateStreaming(
        fetch("game.wasm"),
        {
            "env": {
                "js_sin": Math.sin,
                "js_cos": Math.cos
            }
        });
    const memoryView = new Uint8Array(game.instance.exports.memory.buffer);

    game.instance.exports.init();
    const displayAddr = game.instance.exports.get_display();
    const displayWidth = game.instance.exports.get_display_width();
    const displayHeight = game.instance.exports.get_display_height();
    const displaySize = displayWidth * displayHeight;

    const gameCanvas = document.getElementById("game-canvas");
    document.addEventListener('keydown', e => {
        console.log(e);
        if (e.code === 'Space') {
            game.instance.exports.toggle_pause();
        }
    });
    gameCanvas.addEventListener('mousemove', e => {
        game.instance.exports.mouse_move(e.offsetX, e.offsetY);
    });
    gameCanvas.addEventListener('mousedown', e => {
        game.instance.exports.mouse_click();
    });

    const ctx = gameCanvas.getContext('2d');

    let start;
    function step(timestamp) {
        if (start === undefined) {
            start = timestamp;
        }
        const dt = (timestamp - start) * 0.001;
        start = timestamp;

        game.instance.exports.next_frame(dt);
        const frame = new ImageData(
            new Uint8ClampedArray(
                memoryView.subarray(
                    displayAddr,
                    displayAddr + 4 * displaySize)),
            displayWidth, displayHeight);
        ctx.putImageData(frame, 0, 0);

        window.requestAnimationFrame(step);
    }
    window.requestAnimationFrame(step);
}

start().catch((e) => console.log(e));
