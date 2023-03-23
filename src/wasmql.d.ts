type CodecInfo = {
    endpoint: string;
    wasm: string;
}

interface Codec {
    readonly [name: string]: (...args: any[]) => Promise<any>;
}

export default function<T extends Codec>(info: CodecInfo): Promise<T>;

// type Item = {
//     id: number,
//     name: String,
//     desc: String,
//     done: boolean,
// };

// interface MyCodec extends Codec {
//     readonly create_item: (name: String, data: String) => Promise<Item>;
// }
