export default function Authenticated({ user }) {
    return (
        <div>
            <p>You are authenticated {user.email}</p>
        </div>
    )
}