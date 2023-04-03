import '@/styles/globals.css'
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import type { AppProps } from 'next/app'
import "vis-timeline/styles/vis-timeline-graph2d.css";

export default function App({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />
}
