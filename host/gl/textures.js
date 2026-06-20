// GL texture objects, query objects, and pixel read/write bindings.

import { state } from '../runtime/state.js';
import { getArray } from '../runtime/utils.js';
import { GL, _glGenObject } from '../runtime/registry.js';
import { texture_size } from '../runtime/canvas.js';

export const glTextureBindings = {
    glGenTextures: function (n, textures) {
        _glGenObject(n, textures, "createTexture", GL.textures, "glGenTextures");
    },
    glActiveTexture: function (texture) {
        state.gl.activeTexture(texture);
    },
    glBindTexture: function (target, texture) {
        GL.validateGLObjectID(GL.textures, texture, 'glBindTexture', 'texture');
        state.gl.bindTexture(target, GL.textures[texture]);
    },
    glTexImage2D: function (target, level, internalFormat, width, height, border, format, type, pixels) {
        state.gl.texImage2D(target, level, internalFormat, width, height, border, format, type,
            pixels ? getArray(pixels, Uint8Array, texture_size(internalFormat, width, height)) : null);
    },
    glTexSubImage2D: function (target, level, xoffset, yoffset, width, height, format, type, pixels) {
        state.gl.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type,
            pixels ? getArray(pixels, Uint8Array, texture_size(format, width, height)) : null);
    },
    glReadPixels: function (x, y, width, height, format, type, pixels) {
        var pixelData = getArray(pixels, Uint8Array, texture_size(format, width, height));
        state.gl.readPixels(x, y, width, height, format, type, pixelData);
    },
    glTexParameteri: function (target, pname, param) {
        state.gl.texParameteri(target, pname, param);
    },
    glCopyTexImage2D: function (target, level, internalformat, x, y, width, height, border) {
        state.gl.copyTexImage2D(target, level, internalformat, x, y, width, height, border);
    },
    glDeleteTextures: function (n, textures) {
        for (var i = 0; i < n; i++) {
            var id = getArray(textures + i * 4, Uint32Array, 1)[0];
            var texture = GL.textures[id];
            if (!texture) continue; // GL spec: "glDeleteTextures silently ignores 0s and names that do not correspond to existing textures".
            state.gl.deleteTexture(texture);
            texture.name = 0;
            GL.textures[id] = null;
        }
    },
    glGenQueries: function (n, ids) {
        _glGenObject(n, ids, 'createQuery', GL.timerQueries, 'glGenQueries');
    },
    glDeleteQueries: function (n, ids) {
        for (var i = 0; i < n; i++) {
            var id = getArray(ids + i * 4, Uint32Array, 1)[0];
            var query = GL.timerQueries[id];
            if (!query) {
                continue;
            }
            state.gl.deleteQuery(query);
            query.name = 0;
            GL.timerQueries[id] = null;
        }
    },
    glBeginQuery: function (target, id) {
        GL.validateGLObjectID(GL.timerQueries, id, 'glBeginQuery', 'id');
        state.gl.beginQuery(target, GL.timerQueries[id]);
    },
    glEndQuery: function (target) {
        state.gl.endQuery(target);
    },
    glGetQueryObjectiv: function (id, pname, ptr) {
        GL.validateGLObjectID(GL.timerQueries, id, 'glGetQueryObjectiv', 'id');
        let result = state.gl.getQueryObject(GL.timerQueries[id], pname);
        getArray(ptr, Uint32Array, 1)[0] = result;
    },
    glGetQueryObjectui64v: function (id, pname, ptr) {
        GL.validateGLObjectID(GL.timerQueries, id, 'glGetQueryObjectui64v', 'id');
        let result = state.gl.getQueryObject(GL.timerQueries[id], pname);
        let heap = getArray(ptr, Uint32Array, 2);
        heap[0] = result;
        heap[1] = (result - heap[0]) / 4294967296;
    },
};
