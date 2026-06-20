// System-level env bindings: console, clipboard, RNG, time, canvas dimensions.

import { state } from '../runtime/state.js';
import { UTF8ToString } from '../runtime/utils.js';
import { dpi_scale } from '../runtime/canvas.js';

export const systemBindings = {
    console_debug: function (ptr) {
        console.debug(UTF8ToString(ptr));
    },
    console_log: function (ptr) {
        console.log(UTF8ToString(ptr));
    },
    console_info: function (ptr) {
        console.info(UTF8ToString(ptr));
    },
    console_warn: function (ptr) {
        console.warn(UTF8ToString(ptr));
    },
    console_error: function (ptr) {
        console.error(UTF8ToString(ptr));
    },
    set_emscripten_shader_hack: function (flag) {
        state.emscripten_shaders_hack = flag;
    },
    sapp_set_clipboard: function (ptr, len) {
        state.clipboard = UTF8ToString(ptr, len);
    },
    dpi_scale,
    rand: function () {
        return Math.floor(Math.random() * 2147483647);
    },
    now: function () {
        return Date.now() / 1000.0;
    },
    canvas_width: function () {
        return Math.floor(state.canvas.width);
    },
    canvas_height: function () {
        return Math.floor(state.canvas.height);
    },
    device_pixel_ratio: function () {
        return window.devicePixelRatio || 1.0;
    },
};
