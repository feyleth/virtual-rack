import { For } from "solid-js"
import styles from "./Node.module.css"
import Port from "./Port"
import { Direction, Node } from "../api/node"
import { Store } from "solid-js/store"
import { Item, Menu, useContextMenu } from "./ContextMenu"

type Props = {
    node: Store<Node>
}

export default (props: Props) => {

    let { show, hide } = useContextMenu((<Menu>
        <Item onclick={() => { console.log("test"); hide() }}>
            select
        </Item>
    </Menu>))

    return (
        <div class={styles.node} oncontextmenu={(e) => show(e)}>
            <h1 class={styles.name}>
                {props.node.name}
            </h1>
            <div>
                <For each={Array.from(props.node.ports).filter((el) => el.direction === Direction.In)}>
                    {(port) => (
                        <Port port={port} direction="in" />
                    )}
                </For>
            </div>
            <div class={styles.out}>

                <For each={Array.from(props.node.ports).filter((el) => el.direction === Direction.Out)}>
                    {(port) => (
                        <Port port={port} direction="out" />
                    )}
                </For>

            </div>
        </div >
    )
}
