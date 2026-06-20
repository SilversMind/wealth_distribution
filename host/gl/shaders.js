// GL shader compilation, program linking, and uniform upload bindings.

import { state } from '../runtime/state.js';
import { getArray, UTF8ToString } from '../runtime/utils.js';
import { GL } from '../runtime/registry.js';

export const glShaderBindings = {
    glCreateShader: function (shaderType) {
        var id = GL.getNewId(GL.shaders);
        GL.shaders[id] = state.gl.createShader(shaderType);
        return id;
    },
    glShaderSource: function (shader, count, string, length) {
        GL.validateGLObjectID(GL.shaders, shader, 'glShaderSource', 'shader');
        var source = GL.getSource(shader, count, string, length);

        // https://github.com/emscripten-core/emscripten/blob/incoming/src/library_webgl.js#L2708
        if (state.emscripten_shaders_hack) {
            source = source.replace(/#extension GL_OES_standard_derivatives : enable/g, "");
            source = source.replace(/#extension GL_EXT_shader_texture_lod : enable/g, '');
            var prelude = '';
            if (source.indexOf('gl_FragColor') != -1) {
                prelude += 'out mediump vec4 GL_FragColor;\n';
                source = source.replace(/gl_FragColor/g, 'GL_FragColor');
            }
            if (source.indexOf('attribute') != -1) {
                source = source.replace(/attribute/g, 'in');
                source = source.replace(/varying/g, 'out');
            } else {
                source = source.replace(/varying/g, 'in');
            }

            source = source.replace(/textureCubeLodEXT/g, 'textureCubeLod');
            source = source.replace(/texture2DLodEXT/g, 'texture2DLod');
            source = source.replace(/texture2DProjLodEXT/g, 'texture2DProjLod');
            source = source.replace(/texture2DGradEXT/g, 'texture2DGrad');
            source = source.replace(/texture2DProjGradEXT/g, 'texture2DProjGrad');
            source = source.replace(/textureCubeGradEXT/g, 'textureCubeGrad');

            source = source.replace(/textureCube/g, 'texture');
            source = source.replace(/texture1D/g, 'texture');
            source = source.replace(/texture2D/g, 'texture');
            source = source.replace(/texture3D/g, 'texture');
            source = source.replace(/#version 100/g, '#version 300 es\n' + prelude);
        }

        state.gl.shaderSource(GL.shaders[shader], source);
    },
    glCompileShader: function (shader, count, string, length) {
        GL.validateGLObjectID(GL.shaders, shader, 'glCompileShader', 'shader');
        state.gl.compileShader(GL.shaders[shader]);
    },
    glGetShaderiv: function (shader, pname, p) {
        if (!p) {
            console.error('GL_INVALID_VALUE in glGetShaderiv: null out pointer');
            return;
        }
        GL.validateGLObjectID(GL.shaders, shader, 'glGetShaderiv', 'shader');
        if (pname == 0x8B84) { // GL_INFO_LOG_LENGTH
            var log = state.gl.getShaderInfoLog(GL.shaders[shader]);
            if (log === null) {
                console.error('GL_INVALID_OPERATION in glGetShaderiv: getShaderInfoLog returned null');
                return;
            }
            getArray(p, Int32Array, 1)[0] = log.length + 1;
        } else if (pname == 0x8B88) { // GL_SHADER_SOURCE_LENGTH
            var source = state.gl.getShaderSource(GL.shaders[shader]);
            var sourceLength = (source === null || source.length == 0) ? 0 : source.length + 1;
            getArray(p, Int32Array, 1)[0] = sourceLength;
        } else {
            getArray(p, Int32Array, 1)[0] = state.gl.getShaderParameter(GL.shaders[shader], pname);
        }
    },
    glGetShaderInfoLog: function (shader, maxLength, length, infoLog) {
        GL.validateGLObjectID(GL.shaders, shader, 'glGetShaderInfoLog', 'shader');
        var log = state.gl.getShaderInfoLog(GL.shaders[shader]);
        if (log === null) {
            console.error('GL_INVALID_OPERATION in glGetShaderInfoLog: getShaderInfoLog returned null');
            return;
        }
        let array = getArray(infoLog, Uint8Array, maxLength);
        for (var i = 0; i < maxLength; i++) {
            array[i] = log.charCodeAt(i);
        }
    },
    glDeleteShader: function (shader) { state.gl.deleteShader(shader); },
    glCreateProgram: function () {
        var id = GL.getNewId(GL.programs);
        var program = state.gl.createProgram();
        program.name = id;
        GL.programs[id] = program;
        return id;
    },
    glAttachShader: function (program, shader) {
        GL.validateGLObjectID(GL.programs, program, 'glAttachShader', 'program');
        GL.validateGLObjectID(GL.shaders, shader, 'glAttachShader', 'shader');
        state.gl.attachShader(GL.programs[program], GL.shaders[shader]);
    },
    glLinkProgram: function (program) {
        GL.validateGLObjectID(GL.programs, program, 'glLinkProgram', 'program');
        state.gl.linkProgram(GL.programs[program]);
        GL.populateUniformTable(program);
    },
    glUseProgram: function (program) {
        GL.validateGLObjectID(GL.programs, program, 'glUseProgram', 'program');
        state.gl.useProgram(GL.programs[program]);
    },
    glGetProgramiv: function (program, pname, p) {
        if (!p) {
            console.error('GL_INVALID_VALUE in glGetProgramiv: null out pointer');
            return;
        }
        GL.validateGLObjectID(GL.programs, program, 'glGetProgramiv', 'program');
        if (program >= GL.counter) {
            console.error("GL_INVALID_VALUE in glGetProgramiv");
            return;
        }
        var ptable = GL.programInfos[program];
        if (!ptable) {
            console.error('GL_INVALID_OPERATION in glGetProgramiv(program=' + program + ', pname=' + pname + ', p=0x' + p.toString(16) + '): The specified GL object name does not refer to a program object!');
            return;
        }
        if (pname == 0x8B84) { // GL_INFO_LOG_LENGTH
            var log = state.gl.getProgramInfoLog(GL.programs[program]);
            if (log === null) {
                console.error('GL_INVALID_OPERATION in glGetProgramiv: getProgramInfoLog returned null');
                return;
            }
            getArray(p, Int32Array, 1)[0] = log.length + 1;
        } else if (pname == 0x8B87 /* GL_ACTIVE_UNIFORM_MAX_LENGTH */) {
            console.error("unsupported operation");
            return;
        } else if (pname == 0x8B8A /* GL_ACTIVE_ATTRIBUTE_MAX_LENGTH */) {
            console.error("unsupported operation");
            return;
        } else if (pname == 0x8A35 /* GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH */) {
            console.error("unsupported operation");
            return;
        } else {
            getArray(p, Int32Array, 1)[0] = state.gl.getProgramParameter(GL.programs[program], pname);
        }
    },
    glGetProgramInfoLog: function (program, maxLength, length, infoLog) {
        GL.validateGLObjectID(GL.programs, program, 'glGetProgramInfoLog', 'program');
        var log = state.gl.getProgramInfoLog(GL.programs[program]);
        if (log === null) {
            console.error('GL_INVALID_OPERATION in glGetProgramInfoLog: getProgramInfoLog returned null');
            return;
        }
        let array = getArray(infoLog, Uint8Array, maxLength);
        for (var i = 0; i < maxLength; i++) {
            array[i] = log.charCodeAt(i);
        }
    },
    glGetUniformLocation: function (program, name) {
        GL.validateGLObjectID(GL.programs, program, 'glGetUniformLocation', 'program');
        name = UTF8ToString(name);
        var arrayIndex = 0;
        // If user passed an array accessor "[index]", parse the array index off the accessor.
        if (name[name.length - 1] == ']') {
            var leftBrace = name.lastIndexOf('[');
            arrayIndex = name[leftBrace + 1] != ']' ? parseInt(name.slice(leftBrace + 1)) : 0; // "index]", parseInt will ignore the ']' at the end; but treat "foo[]" as "foo[0]"
            name = name.slice(0, leftBrace);
        }

        var uniformInfo = GL.programInfos[program] && GL.programInfos[program].uniforms[name]; // returns pair [ dimension_of_uniform_array, uniform_location ]
        if (uniformInfo && arrayIndex >= 0 && arrayIndex < uniformInfo[0]) { // Check if user asked for an out-of-bounds element, i.e. for 'vec4 colors[3];' user could ask for 'colors[10]' which should return -1.
            return uniformInfo[1] + arrayIndex;
        } else {
            return -1;
        }
    },
    glUniform1f: function (location, v0) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1f', 'location');
        state.gl.uniform1f(GL.uniforms[location], v0);
    },
    glUniform1i: function (location, v0) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1i', 'location');
        state.gl.uniform1i(GL.uniforms[location], v0);
    },
    glUniform1fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1fv', 'location');
        var view = getArray(value, Float32Array, 1 * count);
        state.gl.uniform1fv(GL.uniforms[location], view);
    },
    glUniform2fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform2fv', 'location');
        var view = getArray(value, Float32Array, 2 * count);
        state.gl.uniform2fv(GL.uniforms[location], view);
    },
    glUniform3fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform3fv', 'location');
        var view = getArray(value, Float32Array, 3 * count);
        state.gl.uniform3fv(GL.uniforms[location], view);
    },
    glUniform4fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform4fv', 'location');
        var view = getArray(value, Float32Array, 4 * count);
        state.gl.uniform4fv(GL.uniforms[location], view);
    },
    glUniform1iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1iv', 'location');
        var view = getArray(value, Int32Array, 1 * count);
        state.gl.uniform1iv(GL.uniforms[location], view);
    },
    glUniform2iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform2iv', 'location');
        var view = getArray(value, Int32Array, 2 * count);
        state.gl.uniform2iv(GL.uniforms[location], view);
    },
    glUniform3iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform3iv', 'location');
        var view = getArray(value, Int32Array, 3 * count);
        state.gl.uniform3iv(GL.uniforms[location], view);
    },
    glUniform4iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform4iv', 'location');
        var view = getArray(value, Int32Array, 4 * count);
        state.gl.uniform4iv(GL.uniforms[location], view);
    },
    glUniformMatrix4fv: function (location, count, transpose, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniformMatrix4fv', 'location');
        var view = getArray(value, Float32Array, 16);
        state.gl.uniformMatrix4fv(GL.uniforms[location], !!transpose, view);
    },
};
