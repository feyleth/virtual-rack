import { Port } from "../api/node"
import { Store } from "solid-js/store"


type Props = {
    direction: "in" | "out"
    port: Store<Port>
}

export default function(props: Props) {
    return (
        <div>
            {props.direction == "in" ? (<>&bull;</>) : null} {props.port.name} {props.direction == "out" ? (<>&bull;</>) : null}
        </div>
    )
}
