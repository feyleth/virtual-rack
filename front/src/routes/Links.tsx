import { onCleanup } from "solid-js";
import { watch_state } from "../api"
import "./Links.css"
import Node from "../components/Node.tsx"


export default () => {
    let events = watch_state();
    onCleanup(() => {
        events.close()
    })
    return (<>
        <svg viewBox="0 0 1000 1000">
            <Node x={100} y={100}></Node>
        </svg>
    </>)
}
