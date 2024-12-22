import { createEffect, createSignal, JSX, Show } from "solid-js"
import style from "./ContextMenu.module.css"

let [menu, setMenu] = createSignal<JSX.Element | undefined>(undefined);
let [position, setPosition] = createSignal<{ x: number, y: number } | undefined>(undefined);

export const useContextMenu = (menu: JSX.Element) => {
    return {
        show: (e: MouseEvent) => {
            e.preventDefault()
            setPosition({ x: e.x, y: e.y })
            setMenu(menu);
        },
        hide: () => { setMenu(undefined) }
    }
}

export default () => {
    let focus !: HTMLDivElement;
    createEffect(() => {
        if (menu() !== undefined) {
            focus.focus()
        } else {
            focus.blur()
        }
    })
    return (<div
        tabIndex="-1"
        ref={focus}
        class={style.global}
        onfocusout={() => { setMenu(undefined) }}
        style={{ left: position()?.x + "px", top: position()?.y + "px" }}>
        <Show when={menu() !== undefined}>
            {menu()}
        </Show>
    </div >)
}

type MenuProps = {
    children?: JSX.Element
}
export const Menu = (props: MenuProps) => {
    return <ul class={style.menu}>{props.children}</ul>
}

type ItemProps = {
    children?: JSX.Element
    onclick?: JSX.EventHandlerUnion<HTMLLIElement, MouseEvent>
}
export const Item = (props: ItemProps) => {
    return <li onclick={props.onclick}>{props.children}</li>
}

type SubMenuProps = {
    children?: JSX.Element
}
export const SubMenu = (props: SubMenuProps) => {
    return <ul>{props.children}</ul>
}
