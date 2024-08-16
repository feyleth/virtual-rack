/* @refresh reload */
import { render } from 'solid-js/web'

import './index.css'
import App from './App'
import { Route, Router } from '@solidjs/router'
import { lazy } from 'solid-js'


const root = document.getElementById('root')

render(() =>
    <Router root={App}>
        <Route path="/" component={lazy(() => import("./routes/Index.tsx"))} />
        <Route path="/links" component={lazy(() => import("./routes/Links.tsx"))} />
    </Router>
    , root!)
