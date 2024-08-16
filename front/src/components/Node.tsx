import { onMount } from "solid-js";

interface Position {
    x: number,
    y: number
}

export default (position: Position) => {
    let textRef: SVGTextElement | undefined;
    let rectRef: SVGRectElement | undefined;
    let border = 2;
    let margin = 10;

    onMount(() => {
        let safeText = textRef!;
        let safeRect = rectRef!;
        let size = safeText.getBBox();
        safeRect.style.width = (size.width + margin * 2 + border * 2) + "px"
        safeRect.style.height = (size.height + margin * 2 + border * 2) + "px"

        safeText.style.transform = "translate(" + (position.x + margin + size.width / 2 + border) + "px, " + (position.y + margin + border) + "px )"
    })
    return (
        <g font-size="3rem">
            <rect ref={rectRef!} y={position.y + "px"} x={position.x + "px"} fill="none" stroke-width={border} stroke="black">
            </rect>
            <text ref={textRef!} text-anchor="middle" dominant-baseline="hanging" >eqsdqsd sqddqsd qsd qsd qsd qsd</text>
        </g >
    )
}
