// Plugin system: register_plugins, init_plugins, miniquad_add_plugin, u32_to_semver

import { state } from './state.js';

export function u32_to_semver(crate_version) {
    let major_version = (crate_version >> 24) & 0xff;
    let minor_version = (crate_version >> 16) & 0xff;
    let patch_version = crate_version & 0xffff;
    return major_version + "." + minor_version + "." + patch_version;
}

export function register_plugins(plugins, importObject) {
    if (plugins == undefined)
        return;

    for (var i = 0; i < plugins.length; i++) {
        if (plugins[i].register_plugin != undefined && plugins[i].register_plugin != null) {
            plugins[i].register_plugin(importObject);
        }
    }
}

export function init_plugins(plugins) {
    if (plugins == undefined)
        return;

    for (var i = 0; i < plugins.length; i++) {
        if (plugins[i].on_init != undefined && plugins[i].on_init != null) {
            plugins[i].on_init();
        }
        if (plugins[i].name == undefined || plugins[i].name == null ||
            plugins[i].version == undefined || plugins[i].version == null) {
            console.warn("Some of the registred plugins do not have name or version");
            console.warn("Probably old version of the plugin used");
        } else {
            var version_func = plugins[i].name + "_crate_version";

            if (state.wasm_exports[version_func] == undefined) {
                console.log("Plugin " + plugins[i].name + " is present in JS bundle, but is not used in the rust code.");
            } else {
                var crate_version = u32_to_semver(state.wasm_exports[version_func]());

                if (plugins[i].version != crate_version) {
                    console.error("Plugin " + plugins[i].name + " version mismatch" +
                                  "js version: " + plugins[i].version + ", crate version: " + crate_version);
                }
            }
        }
    }
}

export function miniquad_add_plugin(plugin) {
    state.plugins.push(plugin);
}
