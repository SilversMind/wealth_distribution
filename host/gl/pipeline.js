// GL pipeline state: clear, blend, depth, stencil, cull, viewport, draw calls.

import { state } from '../runtime/state.js';
import { _webglGet } from '../runtime/registry.js';

export const glPipelineBindings = {
    glClear: function (mask) {
        state.gl.clear(mask);
    },
    glClearColor: function (r, g, b, a) {
        state.gl.clearColor(r, g, b, a);
    },
    glClearDepthf: function (depth) {
        state.gl.clearDepth(depth);
    },
    glClearStencil: function (s) {
        state.gl.clearStencil(s);
    },
    glColorMask: function (red, green, blue, alpha) {
        state.gl.colorMask(red, green, blue, alpha);
    },
    glEnable: function (cap) {
        state.gl.enable(cap);
    },
    glDisable: function (cap) {
        state.gl.disable(cap);
    },
    glBlendFunc: function (sfactor, dfactor) {
        state.gl.blendFunc(sfactor, dfactor);
    },
    glBlendFuncSeparate: function (sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) {
        state.gl.blendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha);
    },
    glBlendEquationSeparate: function (modeRGB, modeAlpha) {
        state.gl.blendEquationSeparate(modeRGB, modeAlpha);
    },
    glDepthFunc: function (func) {
        state.gl.depthFunc(func);
    },
    glFrontFace: function (mode) {
        state.gl.frontFace(mode);
    },
    glCullFace: function (mode) {
        state.gl.cullFace(mode);
    },
    glStencilFuncSeparate: function (face, func, ref_, mask) {
        state.gl.stencilFuncSeparate(face, func, ref_, mask);
    },
    glStencilMaskSeparate: function (face, mask) {
        state.gl.stencilMaskSeparate(face, mask);
    },
    glStencilOpSeparate: function (face, fail, zfail, zpass) {
        state.gl.stencilOpSeparate(face, fail, zfail, zpass);
    },
    glViewport: function (x, y, width, height) {
        state.gl.viewport(x, y, width, height);
    },
    glScissor: function (x, y, w, h) {
        state.gl.scissor(x, y, w, h);
    },
    glPixelStorei: function (pname, param) {
        state.gl.pixelStorei(pname, param);
    },
    glFlush: function () {
        state.gl.flush();
    },
    glFinish: function () {
        state.gl.finish();
    },
    glGetIntegerv: function (name_, p) {
        _webglGet(name_, p, 'EM_FUNC_SIG_PARAM_I');
    },
    glDrawArrays: function (mode, first, count) {
        state.gl.drawArrays(mode, first, count);
    },
    glDrawElements: function (mode, count, type, indices) {
        state.gl.drawElements(mode, count, type, indices);
    },
    glDrawArraysInstanced: function (mode, first, count, primcount) {
        state.gl.drawArraysInstanced(mode, first, count, primcount);
    },
    glDrawElementsInstanced: function (mode, count, type, indices, primcount) {
        state.gl.drawElementsInstanced(mode, count, type, indices, primcount);
    },
};
