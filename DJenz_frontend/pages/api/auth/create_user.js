import axios from 'axios'

export default async function createUser(company, email, password) {
    console.log(email);
    console.log(company);
    console.log(password);
    const create_user = await axios.post('http://localhost:8000/clients/new',
        {
            "company": company,
            "email": email,
            "password": password
        },
        {
            headers: {
                accept: '*/*',
                'Content-Type': 'application/json'
            }
        }).catch(function (error) {
            console.log(error);
            return false;
        })
    return create_user.status == 200;
}
