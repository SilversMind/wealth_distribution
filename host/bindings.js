// Assembles all env.* WASM bindings from focused sub-modules into importObject.

import { glTextureBindings }  from './gl/textures.js';
import { glBufferBindings }   from './gl/buffers.js';
import { glShaderBindings }   from './gl/shaders.js';
import { glPipelineBindings } from './gl/pipeline.js';
import { systemBindings }     from './env/system.js';
import { eventBindings }      from './env/events.js';
import { filesystemBindings } from './env/filesystem.js';

export const importObject = {
    env: {
        ...systemBindings,
        ...glTextureBindings,
        ...glBufferBindings,
        ...glShaderBindings,
        ...glPipelineBindings,
        ...eventBindings,
        ...filesystemBindings,
    }
};
