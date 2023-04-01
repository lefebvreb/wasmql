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

    let classes = (checked) ? "item item-checked" : "item";

    return <div className={classes}>
        <h2>{props.data.name}</h2>
        <p>{props.data.desc}</p>
        <input type="checkbox" checked={checked} onChange={onChecked}></input>
        <button className="bold" onClick={onRemove}>X</button>
    </div>;
}

export default Item;
