import { A } from "@solidjs/router"

import "./Footer.css"

export default () => {
    return (
        <footer>
            <nav>
                <ul>
                    <li><A href="/">Pedal Board</A></li>
                    <li><A href="/links">Links</A></li>
                </ul>
            </nav>
        </footer>
    )
}
