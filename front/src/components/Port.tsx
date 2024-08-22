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
            <span>{props.direction == "in" ? (<>&bull;</>) : null}</span> {props.port.name} <span>{props.direction == "out" ? (<>&bull;</>) : null}</span>
        </div>
    )
}
