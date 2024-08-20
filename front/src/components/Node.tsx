import { onMount } from "solid-js";
import { Position } from "./svg";
import Ports from "./Ports";

type Props = Position & {}

export default (props: Props) => {
    let textRef: SVGTextElement | undefined;
    let rectRef: SVGRectElement | undefined;
    let inRef: SVGGElement | undefined;
    let outRef: SVGGElement | undefined;
    let text = "eqsdqsd sqddqsd qsd qsd qsd qsd";
    let recBorder = 2;
    let rectPadding = 10;
    let portGaps = 10;
    let portMargin = 10;

    onMount(() => {


        // text
        let safeText = textRef!;
        let canvas = document.createElement("canvas");
        let ctx = canvas.getContext("2d")!;
        ctx.font = window.getComputedStyle(safeText, null).getPropertyValue("font");
        let mesureSize = ctx.measureText(text);
        let textSize = { width: mesureSize.width, height: mesureSize.emHeightAscent + mesureSize.emHeightDescent };
        safeText.style.transform = "translate(" + (props.x + rectPadding + textSize.width / 2) + "px, " + (props.y + rectPadding) + "px )"



        //rectangle 
        let safeRect = rectRef!;
        safeRect.style.width = (textSize.width + rectPadding * 2 + recBorder * 2) + "px"
        safeRect.style.height = (textSize.height + rectPadding * 2 + recBorder * 2) + "px"

    })
    return (
        <g font-size="3rem">
            <rect ref={rectRef!} y={props.y + "px"} x={props.x + "px"} fill="none" stroke-width={recBorder} stroke="black">
            </rect>
            <text ref={textRef!} text-anchor="middle" dominant-baseline="hanging" >{text}</text>
            <g ref={inRef}>
                <Ports x={0} y={0} text="Ports" dotPosition="Left" />
            </g>
            <g ref={outRef!}>
            </g>
        </g >
    )
}
