import '../styles/globals.css'
import GnapProvider from '../lib/GnapProvider'

function MyApp({ Component, pageProps }) {
  return (
    <GnapProvider>
      <Component {...pageProps} />
    </GnapProvider>
  )
}

export default MyApp