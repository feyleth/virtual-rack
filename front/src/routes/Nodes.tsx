import { createEffect, createMemo, For, on, onCleanup } from "solid-js";
import { watch_state } from "../api/api.ts"
import styles from "./Nodes.module.css"
import Node from "../components/Node.tsx"
import { NodeTypeDirection } from "../api/node.ts"
import { createStore } from "solid-js/store";
import { State } from "../api/state.ts";


export default () => {
    let events = watch_state();
    let [state, setState] = createStore<State>({ nodes: [], links: [] });
    let [links, setLinks] = createStore<{ from: { top: number, left: number }, to: { top: number, left: number } }[]>([]);
    events.addEventListener("init state", (e) => {
        let state = (JSON.parse(e.data) as State);
        setState(state)
    })

    events.addEventListener("change state", (e) => {
        let state = (JSON.parse(e.data) as State);
        setState(state)
    })
    onCleanup(() => {
        events.close()
    })


    let noPortNode = createMemo(() => state.nodes.filter(node => node.nodeType === NodeTypeDirection.None))
    let inPortNode = createMemo(() => state.nodes.filter(node =>
        node.nodeType === NodeTypeDirection.In
    ));
    let outPortNode = createMemo(() => state.nodes.filter(node =>
        node.nodeType === NodeTypeDirection.Out
    ))
    let bothPortNode = createMemo(() => state.nodes.filter(node =>
        node.nodeType === NodeTypeDirection.Both
    ))
    createEffect(on(() => state.nodes.map(el => el.ports), () => {
        let links = state.links;
        let linksPosition = links.map(link => {
            let portFrom = document.querySelector(`.port-dot[data-id="${link.port_from}"]`)
            let portTo = document.querySelector(`.port-dot[data-id="${link.port_to}"]`)

            if (portFrom === null || portTo === null) {
                console.error("missing port", link.port_from, portFrom, link.port_to, portTo);
                return null
            }

            let computeFrom = portFrom!.getBoundingClientRect();
            let computeTo = portTo!.getBoundingClientRect();

            return { from: { top: computeFrom.top, left: computeFrom.left }, to: { top: computeTo.top, left: computeTo.left } }
        }).filter(el => el !== null)

        setLinks(linksPosition)
    }))

    createEffect(() => { console.log(links.map(el => el)) })


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
