import { useState } from "react";
import { Codec } from "../wasmql";
import { ItemData } from "./Item";

function NewItem(props: {
    codec: Codec,
    addItem: (item: ItemData) => void,
}) {
    let [name, setName] = useState("");
    let [desc, setDesc] = useState("");

    async function createItem() {
        if (name != "") {
            let item = await props.codec.create(name, desc);
            setName("");
            setDesc("");
            props.addItem(item);
        }
    }

    return <div id="sidebar-form">
        <h2>New Item</h2>
        <div>
            <h3>Item name:</h3>
            <input className="form-input bold" type="text" value={name} onChange={e => setName(e.target.value)}/>
        </div>
        <div>
            <h3>Item description:</h3>
            <input className="form-input bold" type="text" value={desc} onChange={e => setDesc(e.target.value)}/>
        </div>
        <button className="bold" id="form-button" onClick={createItem}>New Item</button>
    </div>;
}

export default NewItem;
