// Canvas init, WebGL context + extensions, resize, animation loop, dpi helpers.
// Side effects run at import time (same as original): queries #glcanvas, creates
// WebGL context, acquires extensions, stores results in state.

import { state } from './state.js';

// ---- Canvas and WebGL context setup (side effects at import time) ----

state.canvas = document.querySelector("#glcanvas");
const canvas = state.canvas;

state.gl = canvas.getContext("webgl");
if (state.gl === null) {
    alert("Unable to initialize WebGL. Your browser or machine may not support it.");
}

canvas.focus();

canvas.requestPointerLock = canvas.requestPointerLock ||
    canvas.mozRequestPointerLock ||
    // pointer lock in any form is not supported on iOS safari
    // https://developer.mozilla.org/en-US/docs/Web/API/Pointer_Lock_API#browser_compatibility
    (function () {});
document.exitPointerLock = document.exitPointerLock ||
    document.mozExitPointerLock ||
    // pointer lock in any form is not supported on iOS safari
    (function () {});

function acquireVertexArrayObjectExtension(ctx) {
    // Extension available in WebGL 1 from Firefox 25 and WebKit 536.28/desktop Safari 6.0.3 onwards. Core feature in WebGL 2.
    var ext = ctx.getExtension('OES_vertex_array_object');
    if (ext) {
        ctx['createVertexArray'] = function () { return ext['createVertexArrayOES'](); };
        ctx['deleteVertexArray'] = function (vao) { ext['deleteVertexArrayOES'](vao); };
        ctx['bindVertexArray'] = function (vao) { ext['bindVertexArrayOES'](vao); };
        ctx['isVertexArray'] = function (vao) { return ext['isVertexArrayOES'](vao); };
    } else {
        alert("Unable to get OES_vertex_array_object extension");
    }
}

function acquireInstancedArraysExtension(ctx) {
    // Extension available in WebGL 1 from Firefox 26 and Google Chrome 30 onwards. Core feature in WebGL 2.
    var ext = ctx.getExtension('ANGLE_instanced_arrays');
    if (ext) {
        ctx['vertexAttribDivisor'] = function (index, divisor) { ext['vertexAttribDivisorANGLE'](index, divisor); };
        ctx['drawArraysInstanced'] = function (mode, first, count, primcount) { ext['drawArraysInstancedANGLE'](mode, first, count, primcount); };
        ctx['drawElementsInstanced'] = function (mode, count, type, indices, primcount) { ext['drawElementsInstancedANGLE'](mode, count, type, indices, primcount); };
    }
}

function acquireDisjointTimerQueryExtension(ctx) {
    var ext = ctx.getExtension('EXT_disjoint_timer_query');
    if (ext) {
        ctx['createQuery'] = function () { return ext['createQueryEXT'](); };
        ctx['beginQuery'] = function (target, query) { return ext['beginQueryEXT'](target, query); };
        ctx['endQuery'] = function (target) { return ext['endQueryEXT'](target); };
        ctx['deleteQuery'] = function (query) { ext['deleteQueryEXT'](query); };
        ctx['getQueryObject'] = function (query, pname) { return ext['getQueryObjectEXT'](query, pname); };
    }
}

acquireVertexArrayObjectExtension(state.gl);
acquireInstancedArraysExtension(state.gl);
acquireDisjointTimerQueryExtension(state.gl);

// https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_depth_texture
if (state.gl.getExtension('WEBGL_depth_texture') == null) {
    alert("Cant initialize WEBGL_depth_texture extension");
}

// ---- Exported helpers ----

export function dpi_scale() {
    if (state.high_dpi) {
        return window.devicePixelRatio || 1.0;
    } else {
        return 1.0;
    }
}

export function resize(canvas, on_resize) {
    var dpr = dpi_scale();
    var displayWidth = canvas.clientWidth * dpr;
    var displayHeight = canvas.clientHeight * dpr;

    if (canvas.width != displayWidth ||
        canvas.height != displayHeight) {
        canvas.width = displayWidth;
        canvas.height = displayHeight;
        if (on_resize != undefined)
            on_resize(Math.floor(displayWidth), Math.floor(displayHeight));
    }
}

export function texture_size(internalFormat, width, height) {
    if (internalFormat == state.gl.ALPHA) {
        return width * height;
    } else if (internalFormat == state.gl.RGB) {
        return width * height * 3;
    } else if (internalFormat == state.gl.RGBA) {
        return width * height * 4;
    } else { // TextureFormat::RGB565 | TextureFormat::RGBA4 | TextureFormat::RGBA5551
        return width * height * 3;
    }
}

export function mouse_relative_position(clientX, clientY) {
    var targetRect = canvas.getBoundingClientRect();
    var x = (clientX - targetRect.left) * dpi_scale();
    var y = (clientY - targetRect.top) * dpi_scale();
    return { x, y };
}

export function animation() {
    state.wasm_exports.frame();
    window.requestAnimationFrame(animation);
}
