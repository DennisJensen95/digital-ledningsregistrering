import Layout from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import ContactStyle from '../styles/contact.module.css'

export default function contact() {
    return (
        <Layout>
            <h1 className={utilStyles.title}>
                Kontakt
            </h1>
            <p className={utilStyles.subTitle}>
                Kontakt os omkring information eller tilbud på vores LER 2.0 service. Vores pris afhænger af hvor stor et areal og hvor mange forespørgelser der estimeres i har årligt.
            </p>
            <p className={ContactStyle.email}>
                <a href="mailto:djenzendk@gmail.com">djenzendk@gmail.com</a>
            </p>


        </Layout>
    )
}