import { A } from "@solidjs/router"

import styles from "./Footer.module.css"

export default () => {
    return (
        <footer class={styles.footer}>
            <nav>
                <ul>
                    <li><A href="/">Pedal Board</A></li>
                    <li><A href="/links">Links</A></li>
                </ul>
            </nav>
        </footer>
    )
}
