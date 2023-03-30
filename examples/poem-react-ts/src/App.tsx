import { useState } from "react";
import Item, { ItemData } from "./components/Item";
import NewItem from "./components/NewItem";
import { Codec } from "./wasmql.min";

function App(props: {
    codec: Codec,
}) {
    let [items, setItems] = useState<ItemData[]>([]);

    const addItem = (item: ItemData) => setItems(items.concat(item));

    return <>
        <h1>WasmQL TODO App</h1>
        <h2>Items</h2>
        {items.map((item, i) => <Item key={i} codec={props.codec} data={item}/>)}
        <NewItem codec={props.codec} addItem={addItem}/>
    </>;
}

export default App;
