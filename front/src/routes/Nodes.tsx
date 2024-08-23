import { createEffect, createMemo, For, onCleanup } from "solid-js";
import { watch_state } from "../api/api.ts"
import styles from "./Nodes.module.css"
import Node from "../components/Node.tsx"
import { Node as NodeApi, NodeTypeDirection } from "../api/node.ts"
import { createStore } from "solid-js/store";
import { State } from "../api/state.ts";


export default () => {
    let events = watch_state();
    let [nodeState, setNodeState] = createStore<NodeApi[]>([]);
    events.addEventListener("init state", (e) => {
        let nodes = (JSON.parse(e.data) as State).nodes;
        setNodeState(nodes)
    })

    events.addEventListener("change state", (e) => {
        let nodes = (JSON.parse(e.data) as State).nodes;
        setNodeState(nodes)
        console.log(nodes.length)
    })
    onCleanup(() => {
        events.close()
    })


    let noPortNode = createMemo(() => nodeState.filter(node => node.nodeType === NodeTypeDirection.None))
    let inPortNode = createMemo(() => nodeState.filter(node =>
        node.nodeType === NodeTypeDirection.In
    ));
    let outPortNode = createMemo(() => nodeState.filter(node =>
        node.nodeType === NodeTypeDirection.Out
    ))
    let bothPortNode = createMemo(() => nodeState.filter(node =>
        node.nodeType === NodeTypeDirection.Both
    ))
    createEffect(() => {
        console.log(nodeState)
    })

    return (<div class={styles.nodes}>
        <div class={styles.start}>
            <For each={outPortNode()}>
                {(node) =>
                    (<Node node={node} />)
                }
            </For>
        </div>
        <div class={styles.middle}>
            <For each={[...noPortNode(), ...bothPortNode()]}>
                {(node) =>
                    (<Node node={node} />)
                }
            </For>
        </div>
        <div class={styles.end}>
            <For each={inPortNode()}>
                {(node) =>
                    (<Node node={node} />)
                }
            </For>
        </div>
    </div>)
}
