import { createMemo } from "solid-js";


export type LinkType = { from: { top: number, left: number }, to: { top: number, left: number } };
type Props = {
    link: LinkType
}
export default function(props: Props) {

    let middle = createMemo(() => {
        let link = props.link;
        return { left: (link.to.left - link.from.left) / 2, top: (link.to.top - link.from.top) / 2 }
    })

    return (<path d={`M ${props.link.from.left} ${props.link.from.top} q ${middle().left} 0 ${middle().left} ${middle().top} T ${props.link.to.left} ${props.link.to.top}`} stroke="black" stroke-width="2px" fill="transparent" />)

}
