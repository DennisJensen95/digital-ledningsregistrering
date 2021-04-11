import Head from 'next/head'
import Layout, { siteTitle } from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import Link from 'next/link'
import Date from '../components/date'
import { signIn, signOut, useSession } from 'next-auth/client'

export default function Home({ allPostsData }) {
  const [session, loading] = useSession()

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
      {/* <section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}> */}
      {/* <h2 className={utilStyles.headingLg}>Blog</h2>
        <ul className={utilStyles.list}>
          {allPostsData.map(({ id, date, title }) => (
            <li className={utilStyles.listItem} key={id}>
              <Link href={`/posts/${id}`}>
                <a>{title}</a>
              </Link>
              <br />
              <small className={utilStyles.lightText}>
                <Date dateString={date} />
              </small>
            </li>
          ))}
        </ul> */}
      {/* </section> */}
    </Layout>
  )
}