import { Port } from "../api/node"
import { Store } from "solid-js/store"
import styles from "./Port.module.css"

type Props = {
    direction: "in" | "out"
    port: Store<Port>
}

export default function(props: Props) {
    return (
        <div class={styles.port}>
            {props.direction == "in" ? (<span class="port-dot" data-id={props.port.id}>&bull;</span>) : null} {props.port.name} {props.direction == "out" ? (<span class="port-dot" data-id={props.port.id}>&bull;</span>) : null}
        </div>
    )
}
