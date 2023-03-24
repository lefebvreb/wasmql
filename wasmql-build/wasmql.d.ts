interface CodecInfo {
    endpoint: URL;
    wasm: URL;
}

interface Codec {
    readonly [name: string]: (...args: any[]) => Promise<any>;
}

export default function(info: CodecInfo): Promise<Codec>;