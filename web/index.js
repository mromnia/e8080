const SIZE_X = 224;
const SIZE_Y = 256;

const calcAddrInBuffer = (x, y) => {
    let xAddr = x * (Math.floor(SIZE_Y / 8));

    let yAddr = Math.floor((SIZE_Y - 1 - y) / 8);
    let yBit = (SIZE_Y - 1 - y) % 8;

    let addr = xAddr + yAddr;

    return [addr, yBit];
};

const getBit = (val, bit) => {
    return (val >> bit) & 0x01;
};

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const drawPixel = (imgData, x, y, white) => {
    const addr = (x * 4) + (y * SIZE_X * 4);
    const val = white ? 0xFF : 0x00;
    imgData.data[addr] = imgData.data[addr + 1] = imgData.data[addr + 2] = val;
    imgData.data[addr + 3] = 0xFF;
};

const init = (emulator) => {
    window.emulator = emulator;
    const machine = emulator.instance.exports.am_new();

    const keyToFn = {
        67: emulator.instance.exports.am_coin_key_toggle,
        37: emulator.instance.exports.am_left_p1_key_toggle,
        39: emulator.instance.exports.am_right_p1_key_toggle,
        32: emulator.instance.exports.am_fire_p1_key_toggle,
        83: emulator.instance.exports.am_start_p1_key_toggle,
    };
    const onKeyChange = (key, down) => {
        const fn = keyToFn[key.keyCode];
        if (fn) {
            fn.apply(emulator.instance.exports, [machine, down]);
        }
    };

    document.addEventListener('keydown', (key) => onKeyChange(key, true));
    document.addEventListener('keyup', (key) => onKeyChange(key, false));

    const render = () => {
        const imgData = ctx.createImageData(SIZE_X, SIZE_Y);

        const renderBufferAddr = emulator.instance.exports.am_get_render_buffer(machine);
        const renderBuffer = new Uint8Array(
            emulator.instance.exports.memory.buffer,
            renderBufferAddr,
            0x1c00
        );

        emulator.instance.exports.am_run(machine);

        for (let y = 0; y < (SIZE_Y / 2); ++y) {
            for (let x = 0; x < SIZE_X; ++x) {
                const [addr, bit] = calcAddrInBuffer(x, y);
                const val = getBit(renderBuffer[addr], bit);

                drawPixel(imgData, x, y, val > 0);
            }
        }

        emulator.instance.exports.am_signal_half_render(machine);
        emulator.instance.exports.am_run(machine);

        for (let y = (SIZE_Y / 2); y < SIZE_Y; ++y) {
            for (let x = 0; x < SIZE_X; ++x) {
                const [addr, bit] = calcAddrInBuffer(x, y);
                const val = getBit(renderBuffer[addr], bit);

                drawPixel(imgData, x, y, val > 0);
            }
        }

        emulator.instance.exports.am_signal_finish_render(machine);

        ctx.putImageData(imgData, 0, 0);

        requestAnimationFrame(render);
    };

    render();
};

WebAssembly.instantiateStreaming(fetch('e8080.wasm')).then(init);
