// Shared mutable runtime state — all modules import this object and
// read/write its properties directly.

export const state = {
    wasm_memory: null,
    wasm_exports: null,
    canvas: null,
    gl: null,
    clipboard: null,
    emscripten_shaders_hack: false,
    high_dpi: false,
    plugins: [],
};
