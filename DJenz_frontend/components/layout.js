import Head from 'next/head'
import Image from 'next/image'
import styles from './layout.module.css'
import Link from 'next/link'
import { signIn, signOut, useSession } from 'next-auth/client'

export const siteTitle = 'DJenzen'

export default function Layout({ children, home }) {
    const [session, loading] = useSession()
    return (
        <div>
            <div className={styles.container}>
                <Head>
                    <link rel="icon" href="/logo/favicon.png" />
                    <meta
                        name="Company website DJenzen"
                        content="DJenzen"
                    />
                    <meta
                        property="og:image"
                        content={`https://og-image.vercel.app/${encodeURI(
                            siteTitle
                        )}.png?theme=light&md=0&fontSize=75px&images=https%3A%2F%2Fassets.vercel.com%2Fimage%2Fupload%2Ffront%2Fassets%2Fdesign%2Fnextjs-black-logo.svg`}
                    />
                    <meta name="og:title" content={siteTitle} />
                    <meta name="twitter:card" content="summary_large_image" />
                </Head>
                <nav className={styles.nav_bar}>
                    <div className={styles.links}>
                        <a className={styles.links} href={'/'}>
                            <Image width="164" height="62" src="/logo/facebook_cover_photo_1.png" />
                        </a>
                        {!session && <>
                            <a className={styles.link} href={'/kontakt'}>Kontakt</a>
                            <button className={styles.login_button} onClick={() => signIn()}>Login</button>
                        </>}
                        {session && <>
                            <a className={styles.link} href={'/dashboard'}>Oversigt</a>
                            <a className={styles.link} href={'/profile'}>Profil</a>
                            <a className={styles.link} href={'/kontakt'}>Kontakt</a>
                            <button className={styles.login_button} onClick={() => signOut()}>Logout</button>

                        </>}
                    </div>
                </nav>
            </div>
            <hr className={styles.break_page} />
            <div className={styles.container}>
                <main>{children}</main>
                {
                    !home && (
                        <div className={styles.backToHome}>
                            <Link href="/">
                                <a>← Back to home</a>
                            </Link>
                        </div>
                    )
                }
            </div>
        </div >
    )
}