import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import wasmql, { Codec } from "./wasmql.min.js";

// let codec = await wasmql({
//     endpoint: "/wasmql",
//     wasm: "/codec.wasm",
// });

let codec = {} as Codec;

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App codec={codec}/>
    </React.StrictMode>,
);
