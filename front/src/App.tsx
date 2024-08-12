import { createEffect, createSignal } from 'solid-js'
import solidLogo from './assets/solid.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = createSignal(0)
  createEffect(() => {

    const evtSource = new EventSource("api/state");
    evtSource.onmessage = function(message) {
      console.log(message.data);

    }
    evtSource.onerror = function(err) {
      console.error(err);
    }
  })
  return (
    <>
    </>
  )
}

export default App
