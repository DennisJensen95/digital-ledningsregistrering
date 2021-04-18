import NextAuth from 'next-auth'
import Providers from 'next-auth/providers'
import axios from 'axios'

const callbacks = {
    // Getting the JWT token from API response
    async jwt(token, user) {
        if (user) {
            token.accessToken = user.token
        }

        return token
    },

    async session(session, token) {
        session.accessToken = token.accessToken
        return session
    },
}

const options = {
    site: process.env.NEXTAUTH_URL,
    providers: [
        Providers.Credentials({
            // The name to display on the sign in form (e.g. 'Sign in with...')
            name: 'Credentials',
            // The credentials is used to generate a suitable form on the sign in page.
            // You can specify whatever fields you are expecting to be submitted.
            // e.g. domain, username, password, 2FA token, etc.
            credentials: {
                username: { label: "Username", type: "text", placeholder: "jsmith" },
                password: { label: "Password", type: "password" }
            },
            async authorize(credentials) {
                // Add logic here to look up the user from the credentials supplied

                const user = await axios.post('http://localhost:8000/clients/auth',
                    {
                        "email": credentials.email,
                        "password": credentials.password
                    },
                    {
                        headers: {
                            accept: '*/*',
                            'Content-Type': 'application/json'
                        }
                    })

                if (user.status == 200) {
                    // Any object returned will be saved in `user` property of the JWT
                    console.log("Login succesfull pushing data");
                    console.log(user.data);
                    return user.data
                } else {
                    // If you return null or false then the credentials will be rejected
                    return null
                    // You can also Reject this callback with an Error or with a URL:
                    // throw new Error('error message') // Redirect to error page
                    // throw '/path/to/redirect'        // Redirect to a URL
                }
            }
        })
    ],
    session: {
        jwt: true,
        maxAge: 30 * 24 * 60 * 60 // 30 days
    },
    pages: {
        signIn: '/signin',
        signOut: '/byebye',
        error: '/signin',
    },
    callbacks
}

export default (req, res) => NextAuth(req, res, options)