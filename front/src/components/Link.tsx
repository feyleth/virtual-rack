

export type LinkType = { from: { top: number, left: number }, to: { top: number, left: number } };
type Props = {
    link: LinkType
}
export default function(props: Props) {

    return (<path d={`M ${props.link.from.left} ${props.link.from.top} L ${props.link.to.left} ${props.link.to.top}`} stroke="black" stroke-width="2px" />)

}
