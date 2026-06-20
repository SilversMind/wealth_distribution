// Canvas and window event handler wiring: mouse, keyboard, touch, clipboard, drag-drop, resize.

import { state } from '../runtime/state.js';
import { UTF8ToString, stringToUTF8 } from '../runtime/utils.js';
import { resize, mouse_relative_position, animation } from '../runtime/canvas.js';
import {
    SAPP_EVENTTYPE_TOUCHES_BEGAN,
    SAPP_EVENTTYPE_TOUCHES_MOVED,
    SAPP_EVENTTYPE_TOUCHES_ENDED,
    SAPP_EVENTTYPE_TOUCHES_CANCELLED,
    SAPP_MODIFIER_SHIFT,
    SAPP_MODIFIER_CTRL,
    SAPP_MODIFIER_ALT,
    into_sapp_keycode,
    into_sapp_mousebutton,
} from './input.js';

export const eventBindings = {
    setup_canvas_size: function (high_dpi) {
        // Bug fix: original set window.high_dpi but dpi_scale() read the outer var.
        // Fix: set state.high_dpi so dpi_scale() reads the correct value.
        state.high_dpi = high_dpi;
        resize(state.canvas);
    },
    run_animation_loop: function (ptr) {
        const canvas = state.canvas;
        canvas.focus();

        canvas.onmousemove = function (event) {
            var relative_position = mouse_relative_position(event.clientX, event.clientY);
            var x = relative_position.x;
            var y = relative_position.y;

            // TODO: do not send mouse_move when cursor is captured
            state.wasm_exports.mouse_move(Math.floor(x), Math.floor(y));

            // TODO: check that mouse is captured?
            if (event.movementX != 0 || event.movementY != 0) {
                state.wasm_exports.raw_mouse_move(Math.floor(event.movementX), Math.floor(event.movementY));
            }
        };
        canvas.onmousedown = function (event) {
            canvas.focus();
            event.preventDefault();
            var relative_position = mouse_relative_position(event.clientX, event.clientY);
            var x = relative_position.x;
            var y = relative_position.y;

            var btn = into_sapp_mousebutton(event.button);
            state.wasm_exports.mouse_down(x, y, btn);
        };
        // SO WEB SO CONSISTENT
        canvas.addEventListener('wheel',
            function (event) {
                event.preventDefault();
                state.wasm_exports.mouse_wheel(-event.deltaX, -event.deltaY);
            });
        canvas.onmouseup = function (event) {
            var relative_position = mouse_relative_position(event.clientX, event.clientY);
            var x = relative_position.x;
            var y = relative_position.y;

            var btn = into_sapp_mousebutton(event.button);
            state.wasm_exports.mouse_up(x, y, btn);
        };
        canvas.onkeydown = function (event) {
            var sapp_key_code = into_sapp_keycode(event.code);
            switch (sapp_key_code) {
                //  space, arrows - prevent scrolling of the page
                case 32: case 262: case 263: case 264: case 265:
                // F1-F10
                case 290: case 291: case 292: case 293: case 294: case 295: case 296: case 297: case 298: case 299:
                // backspace is Back on Firefox/Windows
                case 259:
                // tab - for UI
                case 258:
                // quote and slash are Quick Find on Firefox
                case 39: case 47:
                    event.preventDefault();
                    break;
            }

            var modifiers = 0;
            if (event.ctrlKey) {
                modifiers |= SAPP_MODIFIER_CTRL;
            }
            if (event.shiftKey) {
                modifiers |= SAPP_MODIFIER_SHIFT;
            }
            if (event.altKey) {
                modifiers |= SAPP_MODIFIER_ALT;
            }
            state.wasm_exports.key_down(sapp_key_code, modifiers, event.repeat);
            // for "space", "quote", and "slash" preventDefault will prevent
            // key_press event, so send it here instead
            if (sapp_key_code == 32 || sapp_key_code == 39 || sapp_key_code == 47) {
                state.wasm_exports.key_press(sapp_key_code);
            }
        };
        canvas.onkeyup = function (event) {
            var sapp_key_code = into_sapp_keycode(event.code);

            var modifiers = 0;
            if (event.ctrlKey) {
                modifiers |= SAPP_MODIFIER_CTRL;
            }
            if (event.shiftKey) {
                modifiers |= SAPP_MODIFIER_SHIFT;
            }
            if (event.altKey) {
                modifiers |= SAPP_MODIFIER_ALT;
            }

            state.wasm_exports.key_up(sapp_key_code, modifiers);
        };
        canvas.onkeypress = function (event) {
            var sapp_key_code = into_sapp_keycode(event.code);

            // firefox do not send onkeypress events for ctrl+keys and delete key while chrome do
            // workaround to make this behavior consistent
            let chrome_only = sapp_key_code == 261 || event.ctrlKey;
            if (chrome_only == false) {
                state.wasm_exports.key_press(event.charCode);
            }
        };

        canvas.addEventListener("touchstart", function (event) {
            event.preventDefault();

            for (const touch of event.changedTouches) {
                let relative_position = mouse_relative_position(touch.clientX, touch.clientY);
                state.wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_BEGAN, touch.identifier, relative_position.x, relative_position.y);
            }
        });
        canvas.addEventListener("touchend", function (event) {
            event.preventDefault();

            for (const touch of event.changedTouches) {
                let relative_position = mouse_relative_position(touch.clientX, touch.clientY);
                state.wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_ENDED, touch.identifier, relative_position.x, relative_position.y);
            }
        });
        canvas.addEventListener("touchcancel", function (event) {
            event.preventDefault();

            for (const touch of event.changedTouches) {
                let relative_position = mouse_relative_position(touch.clientX, touch.clientY);
                state.wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_CANCELLED, touch.identifier, relative_position.x, relative_position.y);
            }
        });
        canvas.addEventListener("touchmove", function (event) {
            event.preventDefault();

            for (const touch of event.changedTouches) {
                let relative_position = mouse_relative_position(touch.clientX, touch.clientY);
                state.wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_MOVED, touch.identifier, relative_position.x, relative_position.y);
            }
        });

        window.onresize = function () {
            resize(canvas, state.wasm_exports.resize);
        };
        window.addEventListener("copy", function (e) {
            if (state.clipboard != null) {
                e.clipboardData.setData('text/plain', state.clipboard);
                e.preventDefault();
            }
        });
        window.addEventListener("cut", function (e) {
            if (state.clipboard != null) {
                e.clipboardData.setData('text/plain', state.clipboard);
                e.preventDefault();
            }
        });

        window.addEventListener("paste", function (e) {
            e.stopPropagation();
            e.preventDefault();
            var clipboardData = e.clipboardData || window.clipboardData;
            var pastedData = clipboardData.getData('Text');

            if (pastedData != undefined && pastedData != null && pastedData.length != 0) {
                var len = (new TextEncoder().encode(pastedData)).length;
                var msg = state.wasm_exports.allocate_vec_u8(len);
                var heap = new Uint8Array(state.wasm_memory.buffer, msg, len);
                stringToUTF8(pastedData, heap, 0, len);
                state.wasm_exports.on_clipboard_paste(msg, len);
            }
        });

        window.ondragover = function (e) {
            e.preventDefault();
        };

        window.ondrop = async function (e) {
            e.preventDefault();

            state.wasm_exports.on_files_dropped_start();

            for (let file of e.dataTransfer.files) {
                const nameLen = file.name.length;
                const nameVec = state.wasm_exports.allocate_vec_u8(nameLen);
                const nameHeap = new Uint8Array(state.wasm_memory.buffer, nameVec, nameLen);
                stringToUTF8(file.name, nameHeap, 0, nameLen);

                const fileBuf = await file.arrayBuffer();
                const fileLen = fileBuf.byteLength;
                const fileVec = state.wasm_exports.allocate_vec_u8(fileLen);
                const fileHeap = new Uint8Array(state.wasm_memory.buffer, fileVec, fileLen);
                fileHeap.set(new Uint8Array(fileBuf), 0);

                state.wasm_exports.on_file_dropped(nameVec, nameLen, fileVec, fileLen);
            }

            state.wasm_exports.on_files_dropped_finish();
        };

        window.requestAnimationFrame(animation);
    },
    sapp_set_cursor_grab: function (grab) {
        if (grab) {
            state.canvas.requestPointerLock();
        } else {
            document.exitPointerLock();
        }
    },
    sapp_set_cursor: function (ptr, len) {
        state.canvas.style.cursor = UTF8ToString(ptr, len);
    },
    sapp_is_fullscreen: function () {
        let fullscreenElement = document.fullscreenElement;
        return fullscreenElement != null && fullscreenElement.id == state.canvas.id;
    },
    sapp_set_fullscreen: function (fullscreen) {
        if (!fullscreen) {
            document.exitFullscreen();
        } else {
            state.canvas.requestFullscreen();
        }
    },
    sapp_set_window_size: function (new_width, new_height) {
        state.canvas.width = new_width;
        state.canvas.height = new_height;
        resize(state.canvas, state.wasm_exports.resize);
    },
};
