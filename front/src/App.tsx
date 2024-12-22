import { RouteSectionProps } from '@solidjs/router'
import './App.css'
import Footer from './Footer'
import ContextMenu from './components/ContextMenu'


function App(props: RouteSectionProps) {
  return (
    <>
      <ContextMenu></ContextMenu>
      <main>
        {props.children}
      </main>
      <Footer />
    </>
  )
}

export default App
