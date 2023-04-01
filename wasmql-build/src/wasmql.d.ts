interface CodecInfo {
    endpoint: string;
    wasm: string;
}

interface Codec {
    readonly [name: string]: (...args: any[]) => Promise<any>;
}

export default function(info: CodecInfo): Promise<Codec>;