export default async function ({ endpoint, wasm }) {
    // Memory and exported functions of the wasm module instance.
    let exports, memory;

    // Js-owned values table.
    let table = [false, true, null, undefined];

    // Text decoder, to convert between utf-16 (js) and utf-8 (wasm).
    let text_decoder = new TextDecoder();
    let text_encoder = new TextEncoder();
    
    // Appends a new value to the table, returning it's idx.
    function make_value(val) {
        return table.push(val) - 1;
    }

    // Copies some bytes into wasm.
    function copy(bytes) {
        let len = bytes.length;
        let data = exports.__alloc(len);
        let view = new Uint8Array(memory, data, len);
        view.set(bytes);
    }

    // Resets the wasm module and js values table.
    function reset() {
        exports.__reset();
        table.length = 4;
    };

    // Makes a request with the given object, encoder and decoder functions.
    async function query(args, enc_fn, dec_fn) {
        // Turn args into a value.
        let val = make_value(args);
        // Encode value into bytes, and reset module.
        let body = table[enc_fn(val)];
        reset();
        // Perform the requets and extract the resulting bytes.
        let bytes = await fetch(endpoint, { method: 'POST', body })
            .then((res) => res.arrayBuffer());
        // Copy bytes into wasm memory.
        copy(new Uint8Array(bytes));
        // Decode result and reset module.
        let output = table[dec_fn()];
        reset();
        // Return output.
        return output;
    };

    // Imports given to the wasm module instance.
    let imports = {
        /* boolean */
        as_boolean: (val) => table[val],
        /* null */
        is_null: (val) => table[val] === null,
        /* number */
        from_number: (n) => make_value(n),
        as_number: (val) => table[val],
        /* string */
        from_string: (ptr, len) => {
            let view = new Uint8Array(memory, ptr, len);
            return make_value(text_decoder.decode(view));
        },
        as_string: (val) => copy(text_encoder.encode(table[val])),
        as_char: (val) => {
            let str = text_encoder.encode(table[val]);
            let view = new DataView(new ArrayBuffer(4));
            new Uint8Array(view.buffer).set(str.slice(0, 4));
            return view.getUint32();
        },
        /* bytes */
        from_bytes: (data, len) => make_value(memory.slice(data, data + len)),
        as_bytes: (val) => copy(new Uint8Array(table[val])),
        /* object */
        new_object: () => make_value({}),
        object_append: (obj, key, val) => { table[obj][key] = val; },
        /* array */
        new_array: () => make_value([]),
        array_append: (arr, val) => {
            table[arr].push(val);
        },
        array_len: (val) => table[val].length,
        array_get: (val, i) => make_value(table[val][i]),
        /* iter */
        new_iter: (val) => make_value(Object.entries(table[val])),
        iter_key: (val, i) => make_value(table[val][i][0]),
        iter_val: (val, i) => make_value(table[val][i][1]),
        /* throw */
        throw: (val) => {
            let obj = table[val];
            reset();
            throw obj;
        },
        __log: (val) => console.log(table[val]),
    };
    
    // Instantiate wasm module.
    let module = await WebAssembly.instantiateStreaming(fetch(wasm), { env: imports });
    exports = module.instance.exports;
    memory = exports.memory.buffer;

    // Result object.
    let result = {};

    // For each exported function.
    for (let [enc_fn_name, enc_fn] of Object.entries(exports)) {
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