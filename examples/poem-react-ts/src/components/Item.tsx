import { useState } from "react";
import { Codec } from "../wasmql.min";

export type ItemData = {
    id: number,
    name: string,
    desc: string,
    done: boolean,
};

function Item(props: {
    codec: Codec,
    data: ItemData,
    remove: (id: number) => void,
}) {
    let [checked, setChecked] = useState(props.data.done);

    async function onChecked() {
        await props.codec.set_done(props.data.id, !checked);
        setChecked(!checked);
    }

    async function onRemove() {
        await props.codec.remove(props.data.id);
        props.remove(props.data.id);
    }

    return <div>
        <h3>{props.data.name}</h3>
        <p>{props.data.desc}</p>
        <label>
            Done:
            <input type="checkbox" checked={checked} onChange={onChecked}></input>
        </label>
        <button onClick={onRemove}>Remove</button>
    </div>;
}

export default Item;
