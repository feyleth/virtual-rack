import { createEffect, onCleanup } from "solid-js";
import { watch_state } from "../api/api.ts"
import styles from "./Links.module.css"
import Node from "../components/Node.tsx"
import { Node as NodeApi } from "../api/node"
import { createStore } from "solid-js/store";


export default () => {
    let events = watch_state();
    let [nodeState, setNodeState] = createStore<NodeApi[]>([]);
    events.addEventListener("init state", (e) => {
        let nodes = JSON.parse(e.data).nodes as NodeApi[];
        setNodeState(nodes)
    })
    onCleanup(() => {
        events.close()
    })

    createEffect(() => {
        console.log(nodeState)
    })
    return (<div class={styles.nodes}>
        {nodeState[7] !== undefined && (<Node node={nodeState[7]}></Node>)}
    </div>)
}
