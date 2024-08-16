import { RouteSectionProps } from '@solidjs/router'
import './App.css'
import Footer from './Footer'


function App(props: RouteSectionProps) {
  return (
    <>
      <main>
        {props.children}
      </main>
      <Footer />
    </>
  )
}

export default App
