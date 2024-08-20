import { createMemo, For, onCleanup } from "solid-js";
import { watch_state } from "../api/api.ts"
import styles from "./Nodes.module.css"
import Node from "../components/Node.tsx"
import { Direction, Node as NodeApi } from "../api/node.ts"
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

    let noPortNode = createMemo(() => nodeState.filter(node => node.ports.length == 0))
    let inPortNode = createMemo(() => nodeState.filter(node =>
        node.ports.filter(port => port.direction === Direction.In).length > 0 &&
        node.ports.filter(port => port.direction === Direction.Out).length === 0)
    )
    let outPortNode = createMemo(() => nodeState.filter(node =>
        node.ports.filter(port => port.direction === Direction.Out).length > 0 &&
        node.ports.filter(port => port.direction === Direction.In).length === 0)
    )
    let bothPortNode = createMemo(() => nodeState.filter(node =>
        node.ports.filter(port => port.direction === Direction.Out).length > 0 &&
        node.ports.filter(port => port.direction === Direction.In).length > 0)
    )


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
