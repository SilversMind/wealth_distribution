// load wasm module and link with gl functions
//
// this file was made by tons of hacks from emscripten's parseTools and library_webgl
// https://github.com/emscripten-core/emscripten/blob/master/src/parseTools.js
// https://github.com/emscripten-core/emscripten/blob/master/src/library_webgl.js

"use strict";

import { state } from './host/runtime/state.js';
import { importObject } from './host/bindings.js';
import { register_plugins, init_plugins, miniquad_add_plugin, u32_to_semver } from './host/runtime/plugins.js';

const version = "0.3.16";

// read module imports and create fake functions in import object
// this will allow to successfully link wasm even with wrong version of gl.js
// needed to workaround firefox bug with lost error on wasm linking errors
function add_missing_functions_stabs(obj) {
    var imports = WebAssembly.Module.imports(obj);

    for (const i in imports) {
        if (importObject["env"][imports[i].name] == undefined) {
            console.warn("No " + imports[i].name + " function in gl.js");
            importObject["env"][imports[i].name] = function () {
                console.warn("Missed function: " + imports[i].name);
            };
        }
    }
}

export function load(wasm_path) {
    var req = fetch(wasm_path);

    register_plugins(state.plugins, importObject);

    if (typeof WebAssembly.compileStreaming === 'function') {
        WebAssembly.compileStreaming(req)
            .then(obj => {
                add_missing_functions_stabs(obj);
                return WebAssembly.instantiate(obj, importObject);
            })
            .then(obj => {
                state.wasm_memory = obj.exports.memory;
                state.wasm_exports = obj.exports;

                var crate_version = u32_to_semver(state.wasm_exports.crate_version());
                if (version != crate_version) {
                    console.error(
                        "Version mismatch: gl.js version is: " + version +
                            ", miniquad crate version is: " + crate_version);
                }
                init_plugins(state.plugins);
                obj.exports.main();
            })
            .catch(err => {
                console.error("WASM failed to load, probably incompatible gl.js version");
                console.error(err);
            });
    } else {
        req
            .then(function (x) { return x.arrayBuffer(); })
            .then(function (bytes) { return WebAssembly.compile(bytes); })
            .then(function (obj) {
                add_missing_functions_stabs(obj);
                return WebAssembly.instantiate(obj, importObject);
            })
            .then(function (obj) {
                state.wasm_memory = obj.exports.memory;
                state.wasm_exports = obj.exports;

                var crate_version = u32_to_semver(state.wasm_exports.crate_version());
                if (version != crate_version) {
                    console.error(
                        "Version mismatch: gl.js version is: " + version +
                            ", rust sapp-wasm crate version is: " + crate_version);
                }
                init_plugins(state.plugins);
                obj.exports.main();
            })
            .catch(err => {
                console.error("WASM failed to load, probably incompatible gl.js version");
                console.error(err);
            });
    }
}

// Expose miniquad_add_plugin on window for backward compatibility with
// plugins that call window.miniquad_add_plugin() before load() is called.
window.miniquad_add_plugin = miniquad_add_plugin;
