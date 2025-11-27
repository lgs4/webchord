let wasm;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedFloat32ArrayMemory0 = null;

function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

const AudioEngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_audioengine_free(ptr >>> 0, 1));

export class AudioEngine {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        AudioEngineFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_audioengine_free(ptr, 0);
    }
    /**
     * @param {number} cents
     */
    set_detune(cents) {
        wasm.audioengine_set_detune(this.__wbg_ptr, cents);
    }
    /**
     * @param {boolean} enabled
     * @param {number} room_size
     * @param {number} damping
     */
    set_reverb(enabled, room_size, damping) {
        wasm.audioengine_set_reverb(this.__wbg_ptr, enabled, room_size, damping);
    }
    /**
     * @param {boolean} enabled
     * @param {number} rate
     * @param {number} depth
     * @param {number} feedback
     * @param {number} mix
     */
    set_flanger(enabled, rate, depth, feedback, mix) {
        wasm.audioengine_set_flanger(this.__wbg_ptr, enabled, rate, depth, feedback, mix);
    }
    /**
     * @param {boolean} enabled
     * @param {number} rate
     * @param {number} depth
     */
    set_tremolo(enabled, rate, depth) {
        wasm.audioengine_set_tremolo(this.__wbg_ptr, enabled, rate, depth);
    }
    /**
     * @param {number} rate
     */
    set_lfo_rate(rate) {
        wasm.audioengine_set_lfo_rate(this.__wbg_ptr, rate);
    }
    /**
     * @param {number} waveform
     */
    set_waveform(waveform) {
        wasm.audioengine_set_waveform(this.__wbg_ptr, waveform);
    }
    /**
     * @param {number} depth
     */
    set_lfo_depth(depth) {
        wasm.audioengine_set_lfo_depth(this.__wbg_ptr, depth);
    }
    /**
     * @param {number} time_ms
     */
    set_glide_time(time_ms) {
        wasm.audioengine_set_glide_time(this.__wbg_ptr, time_ms);
    }
    /**
     * @returns {number}
     */
    get_sample_rate() {
        const ret = wasm.audioengine_get_sample_rate(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} mode
     */
    set_filter_mode(mode) {
        wasm.audioengine_set_filter_mode(this.__wbg_ptr, mode);
    }
    /**
     * @param {number} volume
     */
    set_live_volume(volume) {
        wasm.audioengine_set_live_volume(this.__wbg_ptr, volume);
    }
    /**
     * @param {number} waveform
     */
    set_lfo_waveform(waveform) {
        wasm.audioengine_set_lfo_waveform(this.__wbg_ptr, waveform);
    }
    /**
     * @param {number} midi_note
     * @param {number} velocity
     */
    timeline_note_on(midi_note, velocity) {
        wasm.audioengine_timeline_note_on(this.__wbg_ptr, midi_note, velocity);
    }
    /**
     * @param {number} cutoff
     */
    set_filter_cutoff(cutoff) {
        wasm.audioengine_set_filter_cutoff(this.__wbg_ptr, cutoff);
    }
    /**
     * @param {boolean} enabled
     */
    set_lfo_to_filter(enabled) {
        wasm.audioengine_set_lfo_to_filter(this.__wbg_ptr, enabled);
    }
    /**
     * @param {number} volume
     */
    set_master_volume(volume) {
        wasm.audioengine_set_master_volume(this.__wbg_ptr, volume);
    }
    /**
     * @param {number} attack
     * @param {number} decay
     * @param {number} sustain
     * @param {number} release
     */
    set_timeline_adsr(attack, decay, sustain, release) {
        wasm.audioengine_set_timeline_adsr(this.__wbg_ptr, attack, decay, sustain, release);
    }
    /**
     * @param {number} midi_note
     */
    timeline_note_off(midi_note) {
        wasm.audioengine_timeline_note_off(this.__wbg_ptr, midi_note);
    }
    /**
     * @param {boolean} enabled
     */
    set_filter_enabled(enabled) {
        wasm.audioengine_set_filter_enabled(this.__wbg_ptr, enabled);
    }
    /**
     * @param {boolean} enabled
     * @param {number} time_ms
     * @param {number} feedback
     * @param {number} mix
     */
    set_timeline_delay(enabled, time_ms, feedback, mix) {
        wasm.audioengine_set_timeline_delay(this.__wbg_ptr, enabled, time_ms, feedback, mix);
    }
    /**
     * @param {number} cents
     */
    set_timeline_detune(cents) {
        wasm.audioengine_set_timeline_detune(this.__wbg_ptr, cents);
    }
    /**
     * @param {boolean} enabled
     * @param {number} room_size
     * @param {number} damping
     */
    set_timeline_reverb(enabled, room_size, damping) {
        wasm.audioengine_set_timeline_reverb(this.__wbg_ptr, enabled, room_size, damping);
    }
    /**
     * @param {number} volume
     */
    set_timeline_volume(volume) {
        wasm.audioengine_set_timeline_volume(this.__wbg_ptr, volume);
    }
    /**
     * @param {number} resonance
     */
    set_filter_resonance(resonance) {
        wasm.audioengine_set_filter_resonance(this.__wbg_ptr, resonance);
    }
    /**
     * @param {boolean} enabled
     * @param {number} rate
     * @param {number} depth
     * @param {number} feedback
     * @param {number} mix
     */
    set_timeline_flanger(enabled, rate, depth, feedback, mix) {
        wasm.audioengine_set_timeline_flanger(this.__wbg_ptr, enabled, rate, depth, feedback, mix);
    }
    /**
     * @param {boolean} enabled
     * @param {number} rate
     * @param {number} depth
     */
    set_timeline_tremolo(enabled, rate, depth) {
        wasm.audioengine_set_timeline_tremolo(this.__wbg_ptr, enabled, rate, depth);
    }
    /**
     * @param {number} rate
     */
    set_timeline_lfo_rate(rate) {
        wasm.audioengine_set_timeline_lfo_rate(this.__wbg_ptr, rate);
    }
    /**
     * @param {number} waveform
     */
    set_timeline_waveform(waveform) {
        wasm.audioengine_set_timeline_waveform(this.__wbg_ptr, waveform);
    }
    /**
     * @param {number} depth
     */
    set_timeline_lfo_depth(depth) {
        wasm.audioengine_set_timeline_lfo_depth(this.__wbg_ptr, depth);
    }
    /**
     * @param {number} time_ms
     */
    set_timeline_glide_time(time_ms) {
        wasm.audioengine_set_timeline_glide_time(this.__wbg_ptr, time_ms);
    }
    stop_all_timeline_notes() {
        wasm.audioengine_stop_all_timeline_notes(this.__wbg_ptr);
    }
    /**
     * @param {number} mode
     */
    set_timeline_filter_mode(mode) {
        wasm.audioengine_set_timeline_filter_mode(this.__wbg_ptr, mode);
    }
    /**
     * @param {number} waveform
     */
    set_timeline_lfo_waveform(waveform) {
        wasm.audioengine_set_timeline_lfo_waveform(this.__wbg_ptr, waveform);
    }
    /**
     * @param {number} cutoff
     */
    set_timeline_filter_cutoff(cutoff) {
        wasm.audioengine_set_timeline_filter_cutoff(this.__wbg_ptr, cutoff);
    }
    /**
     * @param {boolean} enabled
     */
    set_timeline_lfo_to_filter(enabled) {
        wasm.audioengine_set_timeline_lfo_to_filter(this.__wbg_ptr, enabled);
    }
    /**
     * @param {boolean} enabled
     */
    set_timeline_filter_enabled(enabled) {
        wasm.audioengine_set_timeline_filter_enabled(this.__wbg_ptr, enabled);
    }
    /**
     * @param {number} resonance
     */
    set_timeline_filter_resonance(resonance) {
        wasm.audioengine_set_timeline_filter_resonance(this.__wbg_ptr, resonance);
    }
    constructor() {
        const ret = wasm.audioengine_new();
        this.__wbg_ptr = ret >>> 0;
        AudioEngineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} midi_note
     * @param {number} velocity
     */
    note_on(midi_note, velocity) {
        wasm.audioengine_note_on(this.__wbg_ptr, midi_note, velocity);
    }
    /**
     * @param {Float32Array} output
     */
    process(output) {
        var ptr0 = passArrayF32ToWasm0(output, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.audioengine_process(this.__wbg_ptr, ptr0, len0, output);
    }
    /**
     * @param {number} midi_note
     */
    note_off(midi_note) {
        wasm.audioengine_note_off(this.__wbg_ptr, midi_note);
    }
    /**
     * @param {number} attack
     * @param {number} decay
     * @param {number} sustain
     * @param {number} release
     */
    set_adsr(attack, decay, sustain, release) {
        wasm.audioengine_set_adsr(this.__wbg_ptr, attack, decay, sustain, release);
    }
    /**
     * @param {boolean} enabled
     * @param {number} time_ms
     * @param {number} feedback
     * @param {number} mix
     */
    set_delay(enabled, time_ms, feedback, mix) {
        wasm.audioengine_set_delay(this.__wbg_ptr, enabled, time_ms, feedback, mix);
    }
}
if (Symbol.dispose) AudioEngine.prototype[Symbol.dispose] = AudioEngine.prototype.free;

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_copy_to_typed_array_33fbd71146904370 = function(arg0, arg1, arg2) {
        new Uint8Array(arg2.buffer, arg2.byteOffset, arg2.byteLength).set(getArrayU8FromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg___wbindgen_throw_b855445ff6a94295 = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('rust_dsp_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
