import { useEffect, useRef, useState } from "react";
import Item, { ItemData } from "./components/Item";
import NewItem from "./components/NewItem";
import wasmql, { Codec } from "./wasmql.min";

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

    if (!codec_ref.current) {
        return <h1>Loading...</h1>;
    }

    let codec = codec_ref.current!;

    return <>
        <h1>WasmQL TODO App</h1>
        <h2>Items</h2>
        {items.map((item, i) => <Item key={i} remove={removeItem} codec={codec} data={item}/>)}
        <NewItem codec={codec} addItem={addItem}/>
    </>;
}

export default App;
