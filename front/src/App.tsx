import { createEffect } from 'solid-js'
import './App.css'

function App() {
  createEffect(async () => {

    const evtSource = new EventSource("/api/state");
    evtSource.onmessage = function(message) {
      console.log(message.data);

    }
    evtSource.onerror = function(err) {
      console.error(err);
    }

    let req = fetch("/api/test");
    await req
  })
  return (
    <>
    </>
  )
}

export default App
