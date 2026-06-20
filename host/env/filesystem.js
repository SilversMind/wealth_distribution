// Filesystem/XHR bindings: async file loading and buffer transfer to WASM.

import { state } from '../runtime/state.js';
import { UTF8ToString } from '../runtime/utils.js';

// FS state — local to this module
var FS = {
    loaded_files: [],
    unique_id: 0
};

export const filesystemBindings = {
    fs_load_file: function (ptr, len) {
        var url = UTF8ToString(ptr, len);
        var file_id = FS.unique_id;
        FS.unique_id += 1;
        var xhr = new XMLHttpRequest();
        xhr.open('GET', url, true);
        xhr.responseType = 'arraybuffer';

        xhr.onreadystatechange = function () {
            // looks like readyState === 4 will be fired on either successful or unsuccessful load:
            // https://stackoverflow.com/a/19247992
            if (this.readyState === 4) {
                if (this.status === 200) {
                    var uInt8Array = new Uint8Array(this.response);
                    FS.loaded_files[file_id] = uInt8Array;
                    state.wasm_exports.file_loaded(file_id);
                } else {
                    FS.loaded_files[file_id] = null;
                    state.wasm_exports.file_loaded(file_id);
                }
            }
        };
        xhr.send();

        return file_id;
    },

    fs_get_buffer_size: function (file_id) {
        if (FS.loaded_files[file_id] == null) {
            return -1;
        } else {
            return FS.loaded_files[file_id].length;
        }
    },
    fs_take_buffer: function (file_id, ptr, max_length) {
        var file = FS.loaded_files[file_id];
        console.assert(file.length <= max_length);
        var dest = new Uint8Array(state.wasm_memory.buffer, ptr, max_length);
        for (var i = 0; i < file.length; i++) {
            dest[i] = file[i];
        }
        delete FS.loaded_files[file_id];
    },
};
