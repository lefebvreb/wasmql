interface CodecInfo {
    endpoint: URL;
    wasm: Response | PromiseLike<Response>;
}

interface Codec {
    readonly [name: string]: (...args: any[]) => Promise<any>;
}

export default function(info: CodecInfo): Promise<Codec>;