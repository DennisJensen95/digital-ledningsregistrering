import { useSession } from 'next-auth/client'
import Layout from '../components/layout'
import UnauthenticatedComponent from "../components/unauthenticated"
import AuthenticatedComponent from "../components/authenticated"


export default function Profile() {
    const [session, loading] = useSession()
    console.log(session)

    if (loading) {
        return (
            <p>
                Loading
            </p>
        )
    };

    if (!session) return (
        <Layout>
            <UnauthenticatedComponent />
        </Layout>
    )

    return (
        <Layout>
            <AuthenticatedComponent user={session.user} />
        </Layout>
    )
}