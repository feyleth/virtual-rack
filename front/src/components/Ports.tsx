import { onMount } from "solid-js";
import { Position } from "./svg";

type Props = Position & { text: string, dotPosition: "Left" | "Right" }

export default (props: Props) => {
    let textRef: SVGTextElement | undefined;
    let dotRef: SVGCircleElement | undefined;
    let space = 10;

    onMount(() => {
        // text
        let safeText = textRef!;
        let canvas = document.createElement("canvas");
        let ctx = canvas.getContext("2d")!;
        ctx.font = window.getComputedStyle(safeText, null).getPropertyValue("font");
        let mesureSize = ctx.measureText(props.text);
        let textSize = { width: mesureSize.width, height: mesureSize.emHeightAscent + mesureSize.emHeightDescent };

        let dotRadius = textSize.height / 4;

        let textx = props.dotPosition == "Right" ? (props.x + textSize.width / 2) : (props.x + dotRadius * 2 + space + textSize.width / 2)
        safeText.style.transform = "translate(" + textx + "px, " + (props.y) + "px )"

        // dot
        let safeDot = dotRef!;
        let dotx = props.dotPosition == "Right" ? (props.x + textSize.width + space + dotRadius) : (props.x);
        safeDot.style.transform = "translate(" + dotx + "px, " + (textSize.height / 2) + "px)"
        safeDot.setAttribute("r", dotRadius + "")
    })
    return (
        <g font-size="0.75em">
            <text ref={textRef!} text-anchor="middle" dominant-baseline="hanging" >{props.text}</text>
            <circle ref={dotRef!}></circle>
        </g >
    )
}
