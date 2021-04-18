// pages/dashboard.js
import { getSession } from 'next-auth/client'
import Layout from '../components/layout'
import axios from 'axios'
import React, { useState } from 'react';

export default function Dashboard({ user }) {
    const [selectedFile, setSelectedFile] = useState();
    const [isSelected, setIsSelected] = useState(false);

    const changeHandler = (event) => {
        setSelectedFile(event.target.files[0]);
        setIsSelected(true);
    };

    const handleSubmission = async () => {
        const formData = new FormData();
        formData.append('file_attachment', selectedFile);
        console.log("http://localhost:8000/clients/files/upload/" + user.company);
        const response = await axios.post("http://localhost:8000/clients/files/upload/" + user.company, formData, {
            headers: {
                "Content-Type": "multipart/form-data"
            },
        });

        console.log(response);
        if (response.status == 200) {
            alert('Upload Successful');
        } else {
            //if no file selected the show alert
            alert('Please Select File first');
        }

    }

    console.log(user);
    return (
        <Layout>
            <h1>Oversigt</h1>
            <h2>Firma {user.company}</h2>
            <p>Her kan du se besvaret grave forespørgelser og upload jeres gældende data.</p>
            <div>
                <h3>
                    Upload filer
                </h3>
                <input type="file" name="file" onChange={changeHandler} />
                {isSelected ? (
                    <div>
                        <p>Filename: {selectedFile.name}</p>
                        <p>Filetype: {selectedFile.type}</p>
                        <p>Size in bytes: {selectedFile.size}</p>
                        <p>
                            lastModifiedDate:{' '}
                            {selectedFile.lastModifiedDate.toLocaleDateString()}
                        </p>
                    </div>
                ) : (
                    <p>Select a file to show details</p>
                )}
                <div>
                    <button onClick={handleSubmission}>Submit</button>
                </div>
            </div>
        </Layout >
    )
}

export async function getServerSideProps(ctx) {
    const session = await getSession(ctx)
    if (!session) {
        ctx.res.writeHead(302, { Location: '/' })
        ctx.res.end()
        return {}
    }

    const response = await axios.get('http://localhost:8000/clients/' + session.user.email);
    const data = response.data;

    return {
        props: {
            user: data,
        },
    }
}