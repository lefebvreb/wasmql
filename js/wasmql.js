export default async function ({ endpoint, wasm }) {
    // Memory and exported functinos of the wasm module instance.
    let exports, memory;

    // Js-owned values table.
    const table = [false, true, null, undefined];

    // Appends a new value to the table, returning it's idx.
    function value(obj) {
        let idx = table.length;
        table.push(obj);
        return idx;
    }

    // Text decoder, to convert between utf-16 (js) and utf-8 (wasm).
    const text_decoder = new TextDecoder();

    // Copies some bytes into wasm.
    function copy(bytes) {
        const len = bytes.length;
        const ptr = exports.__alloc(bytes.length);
        const view = new Uint8Array(memory, ptr, len);
        view.set(new Uint8Array(bytes));
        return [ptr, len]
    }

    // Resets the wasm module and js values table.
    function reset() {
        exports.reset();
        table.length = 4;
    };

    // Imports given to the wasm module instance.
    const imports = {
        number: (num) => value(num),
        string: (ptr, len) => {
            const view = new Uint8Array(mod.memory.buffer, ptr, len);
            return value(text_decoder(view));
        },
        bytes: (ptr, len) => {
            const view = new Uint8Array(mod.memory.buffer, ptr, len);
            return value(view.buffer);
        },
        object: () => value({}),
        object_append: (obj, key, val) => {
            table[obj][key] = val;
        },
        array: () => value([]),
        array_append: (obj, val) => {
            table[obj].push(val);
        },
        throw: (val) => {
            let obj = table[val];
            reset();
            throw obj;
        },
    };
    
    // Instantiate wasm module.
    const wasm = await WebAssembly.instantiateStreaming(fetch(wasm), { env: imports });
    exports = wasm.instance.exports;
    memory = wasm.memory.buffer;

    // Result object.
    let result = {};

    // Makes a request with the given object, encoder and decoder functions.
    async function query(input, enc_func, dec_func) {
        // Get encoded body and reset module.
        let body = table[enc_func(input)];
        reset();
        // Perform the requets and extract bytes.
        let response = await fetch(endpoint, {
            method: 'POST',
            body,
        });
        let bytes = await response.arrayBuffer();
        // Copy bytes into wasm memory.
        let [ptr, len] = copy(bytes);
        // Decode result and reset module.
        let output = table[dec_func(ptr, len)];
        reset();
        // Return output.
        return output;
    };

    // For each exported function.
    for (let enc_fn of exports) {
        // If the function name starts with 'enc_', i.e. it is an encoding function.
        if (enc_fn.name.startsWith('enc_')) {
            // Name of the original function.
            let name = enc_fn.name.substring(4);
            // Counterpart decoding function.
            let dec_fn = exports['dec_' + name];
            // Make the request function.
            result[name] = (obj) => query(obj, enc_fn, dec_fn);
        }
    }

    return result;
}