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
}) {
    let [checked, setChecked] = useState(props.data.done);

    async function onChecked() {
        await props.codec.set_done(props.data.id, !checked);
        setChecked(!checked);
    }

    return <div>
        <h3>{props.data.name}</h3>
        <p>{props.data.desc}</p>
        <input type="checkbox" checked={checked} onChange={onChecked}></input>
    </div>;
}

export default Item;
