export default async function ({ endpoint, wasm }) {
    // Memory and exported functions of the wasm module instance.
    let exports, memory;

    // Js-owned values table.
    let table = [false, true, null, undefined];

    // Appends a new value to the table, returning it's idx.
    function value(val) {
        return table.push(val) - 1;
    }

    // Text decoder, to convert between utf-16 (js) and utf-8 (wasm).
    let text_decoder = new TextDecoder();

    // Copies some bytes into wasm.
    function copy(bytes) {
        let len = bytes.length;
        let ptr = exports.__alloc(len);
        let view = new Uint8Array(memory, ptr, len);
        view.set(new Uint8Array(bytes));
        return [ptr, len]
    }

    // Resets the wasm module and js values table.
    function reset() {
        exports.__reset();
        table.length = 4;
    };

    // Imports given to the wasm module instance.
    let imports = {
        number: (num) => value(num),
        string: (ptr, len) => {
            let view = new Uint8Array(memory, ptr, len);
            return value(text_decoder.decode(view));
        },
        bytes: (ptr, len) => {
            let view = new Uint8Array(memory, ptr, len);
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
    let module = await WebAssembly.instantiateStreaming(fetch(wasm), { env: imports });
    exports = module.instance.exports;
    memory = exports.memory.buffer;

    // Result object.
    let result = {};

    // Makes a request with the given object, encoder and decoder functions.
    async function query(args, enc_fn, dec_fn) {
        // Turn args into a value.
        let val = value(args);
        // Encode value into bytes, and reset module.
        let body = table[enc_fn(val)];
        reset();
        // Perform the requets and extract the resulting bytes.
        let bytes = await fetch(endpoint, { method: 'POST', body })
            .then((res) => res.arrayBuffer());
        // Copy bytes into wasm memory.
        let [ptr, len] = copy(bytes);
        // Decode result and reset module.
        let output = table[dec_fn(ptr, len)];
        reset();
        // Return output.
        return output;
    };

    // For each exported function.
    for (const [enc_fn_name, enc_fn] of Object.entries(exports)) {
        // If the function name starts with 'enc_', i.e. it is an encoding function.
        if (enc_fn_name.startsWith('enc_')) {
            // Name of the original function.
            let name = enc_fn_name.substring(4);
            // Counterpart decoding function.
            let dec_fn = exports['dec_' + name];
            // Make the request function.
            result[name] = (...args) => query(args, enc_fn, dec_fn);
        }
    }

    return result;
}