import React, { Component } from "react";
import styles from '../styles/login.module.css'
import { signIn } from 'next-auth/client'
import createUser from './api/auth/create_user'


export class Login extends Component {
    constructor(props) {
        super(props);
        this.state = {
            sideActive: styles.sign_container,
            buttonState: styles.sign_button,
            email: "",
            password: "",
            company: "",
            errorMessage: "",
        };

        this.changeToSignIn = this.changeToSignIn.bind(this);
        this.changeToSignUp = this.changeToSignUp.bind(this);
    }

    createUserWithEmailAndPassword = async (event) => {
        event.preventDefault();
        console.log("Email: %s, Password: %s", this.state.email, this.state.password);
    }

    setCompanyName(name) {
        this.setState({ company: name, errorMessage: "" })
    }

    setEmail(email) {
        this.setState({ email: email, errorMessage: "" });
    }

    setPassword(password) {
        this.setState({ password: password, errorMessage: "" });
    }

    changeToSignUp() {
        this.setState({ sideActive: [styles.sign_container, styles.right_panel_active].join(' ') })
    }

    changeToSignIn() {
        this.setState({ sideActive: styles.sign_container })
    }

    validateForm() {
        const state = this.state.email.length > 0 && this.state.password.length > 0;
        const email = this.state.email.includes("@");

        return state && email;
    }

    signUp = async (e) => {
        e.preventDefault()
        const state = await createUser(this.state.company, this.state.email, this.state.password);
        if (state) {
            this.changeToSignIn();
        }
    }

    handleLogin = (e) => {
        e.preventDefault()

        const input = {
            email: this.state.email,
            password: this.state.password,
            callbackUrl: `${window.location.origin}/profile`
        }
        signIn('credentials', input)
    }

    render() {


        return (
            <div className={styles.sign_body}>
                <div className={this.state.sideActive} id="container">
                    <div className={[styles.sign_form_container, styles.sign_up_container].join(' ')}>
                        <form className={styles.sign_form} action="#">
                            <h1 className={styles.sign_h1}>Opret konto</h1>
                            <span className={styles.sign_span}>Indtast din email for at oprette en konto</span>
                            <input onChange={(e) => this.setCompanyName(e.target.value)} className={styles.sign_input} type="text" placeholder="Firma navn" />
                            <input onChange={(e) => this.setEmail(e.target.value)} className={styles.sign_input} type="email" placeholder="Email" />
                            <input onChange={(e) => this.setPassword(e.target.value)} className={styles.sign_input} type="password" placeholder="Password" />
                            <p className={styles.sign_p_error}>{this.state.errorMessage}</p>
                            <button className={styles.sign_button} onClick={event => {
                                this.signUp(event);
                            }} disabled={!this.validateForm()} >Tilmeld</button>
                        </form>
                    </div>
                    <div className={[styles.sign_form_container, styles.sign_in_container].join(' ')}>
                        <form className={styles.sign_form} action="#">
                            <h1 className={styles.sign_h1}>Login</h1>
                            <input onChange={(e) => this.setEmail(e.target.value)} className={styles.sign_input} type="email" placeholder="Email" />
                            <input onChange={(e) => this.setPassword(e.target.value)} className={styles.sign_input} type="password" placeholder="Password" />
                            <a href="/">Glemt kodeord?</a>
                            <button className={styles.sign_button} onClick={event => {
                                this.handleLogin(event);
                            }} disabled={!this.validateForm()}>Login</button>
                        </form>
                    </div>
                    <div className={styles.overlay_container}>
                        <div className={styles.sign_overlay}>
                            <div className={[styles.overlay_panel, styles.overlay_left].join(' ')}>
                                <img width="160" height="160" src="/logo/logo_transparent.png" />
                                <h1 className={styles.sign_h1}>Velkommen tilbage</h1>
                                <p className={styles.sign_p}></p>
                                <button onClick={this.changeToSignIn} className={[styles.sign_button, styles.ghost].join(' ')} id="signIn">Login</button>
                            </div>
                            <div className={[styles.overlay_panel, styles.overlay_right].join(' ')}>
                                <img width="160" height="160" src="/logo/logo_transparent.png" />
                                <h1 className={styles.sign_h1}>Opret konto</h1>
                                <p className={styles.sign_p}></p>
                                <button onClick={this.changeToSignUp} className={[styles.sign_button, styles.ghost].join(' ')} id="signUp">Tilmeld</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div >
        );
    }
}

export default Login;