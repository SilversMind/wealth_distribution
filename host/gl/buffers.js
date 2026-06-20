// GL buffer, VAO, framebuffer, and vertex attribute bindings.

import { state } from '../runtime/state.js';
import { getArray, UTF8ToString } from '../runtime/utils.js';
import { GL, _glGenObject } from '../runtime/registry.js';

export const glBufferBindings = {
    glGenBuffers: function (n, buffers) {
        _glGenObject(n, buffers, 'createBuffer', GL.buffers, 'glGenBuffers');
    },
    glBindBuffer: function (target, buffer) {
        GL.validateGLObjectID(GL.buffers, buffer, 'glBindBuffer', 'buffer');
        state.gl.bindBuffer(target, GL.buffers[buffer]);
    },
    glBufferData: function (target, size, data, usage) {
        state.gl.bufferData(target, data ? getArray(data, Uint8Array, size) : size, usage);
    },
    glBufferSubData: function (target, offset, size, data) {
        state.gl.bufferSubData(target, offset, data ? getArray(data, Uint8Array, size) : size);
    },
    glDeleteBuffers: function (n, buffers) {
        for (var i = 0; i < n; i++) {
            var id = getArray(buffers + i * 4, Uint32Array, 1)[0];
            var buffer = GL.buffers[id];

            // From spec: "glDeleteBuffers silently ignores 0's and names that do not
            // correspond to existing buffer objects."
            if (!buffer) continue;

            state.gl.deleteBuffer(buffer);
            buffer.name = 0;
            GL.buffers[id] = null;
        }
    },
    glGenVertexArrays: function (n, arrays) {
        _glGenObject(n, arrays, 'createVertexArray', GL.vaos, 'glGenVertexArrays');
    },
    glBindVertexArray: function (vao) {
        state.gl.bindVertexArray(GL.vaos[vao]);
    },
    glGenFramebuffers: function (n, ids) {
        _glGenObject(n, ids, 'createFramebuffer', GL.framebuffers, 'glGenFramebuffers');
    },
    glBindFramebuffer: function (target, framebuffer) {
        GL.validateGLObjectID(GL.framebuffers, framebuffer, 'glBindFramebuffer', 'framebuffer');
        state.gl.bindFramebuffer(target, GL.framebuffers[framebuffer]);
    },
    glFramebufferTexture2D: function (target, attachment, textarget, texture, level) {
        GL.validateGLObjectID(GL.textures, texture, 'glFramebufferTexture2D', 'texture');
        state.gl.framebufferTexture2D(target, attachment, textarget, GL.textures[texture], level);
    },
    glDeleteFramebuffers: function (n, buffers) {
        for (var i = 0; i < n; i++) {
            var id = getArray(buffers + i * 4, Uint32Array, 1)[0];
            var buffer = GL.framebuffers[id];

            // From spec: "glDeleteFrameBuffers silently ignores 0's and names that do not
            // correspond to existing buffer objects."
            if (!buffer) continue;

            state.gl.deleteFramebuffer(buffer);
            buffer.name = 0;
            GL.framebuffers[id] = null;
        }
    },
    glGetAttribLocation: function (program, name) {
        return state.gl.getAttribLocation(GL.programs[program], UTF8ToString(name));
    },
    glEnableVertexAttribArray: function (index) {
        state.gl.enableVertexAttribArray(index);
    },
    glDisableVertexAttribArray: function (index) {
        state.gl.disableVertexAttribArray(index);
    },
    glVertexAttribPointer: function (index, size, type, normalized, stride, ptr) {
        state.gl.vertexAttribPointer(index, size, type, !!normalized, stride, ptr);
    },
    glVertexAttribDivisor: function (index, divisor) {
        state.gl.vertexAttribDivisor(index, divisor);
    },
};
