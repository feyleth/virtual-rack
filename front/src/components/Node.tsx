
interface Position {
    x: number,
    y: number
}

export default (position: Position) => {
    return (
        <g>
            <rect width="100px" height="100px" y={position.y} x={position.x} fill="none" stroke-width="4" stroke="black">
            </rect>
            <text style={"transform: translate(calc(" + position.x + "px + 52px) , calc(" + position.y + "px + 1em + 4px))"} text-anchor="middle" font-size="3rem">eqsdqsd sqd </text>
        </g >
    )
}
