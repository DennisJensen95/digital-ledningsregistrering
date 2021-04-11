import Head from 'next/head'
import Layout, { siteTitle } from '../components/layout'
import utilStyles from '../styles/utils.module.css'

export default function Home() {
  return (
    <Layout home>
      <Head>
        <title>{siteTitle}</title>
      </Head>
      <h1 className={utilStyles.title}>
        LER 2.0 service
      </h1>
      <p className={utilStyles.subTitle}>
        Automatisk LER service. Der er ikke længere brug for at svare manuelt på LER forespørgelser. Det kan vi klare for dig!
      </p>
    </Layout>
  )
}