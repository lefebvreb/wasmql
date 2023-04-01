import { useEffect, useRef, useState } from "react";
import Item, { ItemData } from "./components/Item";
import NewItem from "./components/NewItem";
import wasmql, { Codec } from "./wasmql";

function App() {
    let codec_ref = useRef<Codec | undefined>();
    let [items, setItems] = useState<ItemData[]>([]);

    const addItem = (item: ItemData) => setItems(items.concat(item));

    const removeItem = (id: number) => setItems(items.filter((item) => item.id != id));

    useEffect(() => {
        wasmql({
            endpoint: "/wasmql",
            wasm: "/codec.wasm",
        }).then(codec => {
            codec_ref.current = codec;
            codec.items().then(setItems);
        });
    }, []);

    if (codec_ref.current == undefined) {
        return <></>;
    }

    let codec = codec_ref.current!;

    return <>
        <div id="root-sidebar">
            <h1>WasmQL TODO</h1>
            <NewItem codec={codec} addItem={addItem}/>
        </div>
        <div id="root-content">
            <h1>Items</h1>
            {items.map((item) => <Item key={item.id} remove={removeItem} codec={codec} data={item}/>)}
        </div>
    </>;
}

export default App;
