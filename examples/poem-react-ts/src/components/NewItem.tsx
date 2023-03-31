import { useState } from "react";
import { Codec } from "../wasmql.min";
import { ItemData } from "./Item";

function NewItem(props: {
    codec: Codec,
    addItem: (item: ItemData) => void,
}) {
    let [name, setName] = useState("");
    let [desc, setDesc] = useState("");

    async function createItem() {
        if (name && desc) {
            let item = await props.codec.create(name, desc);
            setName("");
            setDesc("");
            props.addItem(item);
        }
    }

    return <div>
        <h2>New Item</h2>
        <label>
            Item name:
            <input type="text" value={name} onChange={e => setName(e.target.value)}/>
        </label>
        <label>
            Item description:
            <input type="text" value={desc} onChange={e => setDesc(e.target.value)}/>
        </label>
        <button onClick={createItem}>New Item</button>
    </div>;
}

export default NewItem;
