import Layout from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import ContactStyle from '../styles/contact.module.css'

export default function byebye() {
    return (
        <Layout>
            <h1 className={utilStyles.title}>
                Farvel!
            </h1>
            <p className={utilStyles.subTitle}>
                Vi håber du mener vores løsning passer dine behov. Hvis ikke kan du altid skrive mangler på nedenstående email.
            </p>
            <p className={ContactStyle.email}>
                <a href="mailto:djenzendk@gmail.com">djenzendk@gmail.com</a>
            </p>
        </Layout>
    )
}