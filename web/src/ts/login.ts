import * as $ from 'jquery';
import { LOGIN_ENDPOINT } from './config';
import { ILoginResponse } from './server_types'; 
import { setCookie      } from './util';
import { isLoggedIn     } from './auth';

interface ILoginForm extends HTMLFormElement {
    username: HTMLInputElement;
    password: HTMLInputElement;
}

export function setupLoginPage() {
    document.getElementById("login-form-submit-btn").addEventListener("click", function(e) {
        submitLoginForm();
    });

    document.getElementById("login-form").addEventListener("submit", function(e) {
        e.preventDefault();
    });

    //Check if the user is already logged in
    isLoggedIn().then((e) => {
        if(e) {
            document.location.href = "dashboard/index.html";
        }
    });
}

function submitLoginForm() {
    const loginForm =       <ILoginForm> document.getElementById('login-form');
    const statusField =     document.getElementById("form-status-field");
    const usernameField =   document.getElementById('login-form-username-field');
    const passwordField =   document.getElementById('login-form-password-field');
    
    if(loginForm.username.value == "") {
        usernameField.classList.value = "border-red";

        statusField.innerHTML = "Please enter your username!";
        statusField.classList.value = "border-red form-status-field";
        statusField.style.visibility = "visible";

        return;
    } else {
        usernameField.classList.value = "";
    }

    if(loginForm.password.value == "") {
        passwordField.classList.value = "border-red";

        statusField.innerHTML = "Please enter your password!";
        statusField.classList.value = "border-red form-status-field";
        statusField.style.visibility = "visible";

        return;
    } else {
        passwordField.classList.value = "";
    }

    statusField.style.visibility = "hidden";
    statusField.innerHTML = "";

    let loginRequest = $.ajax({
        url: LOGIN_ENDPOINT,
        method: 'POST',
        data: {
            username_base64: btoa(loginForm.username.value),
            password_base64: btoa(loginForm.password.value)
        }
    });

    loginRequest.then((e: ILoginResponse) => {
        if(e.status != 200) {
            statusField.innerHTML = "Username and password combination is incorrect, or the account does not exist!";
            statusField.classList.value = "border-red form-status-field";
            statusField.style.visibility = "visible";
        
            return;
        }

        setCookie("sessionid", e.session_id, 30*24*60*60);
        document.location.href = "dashboard/index.html";
    });

    loginRequest.fail((_e) => {
        statusField.innerHTML = "Something went wrong. Please try again later or check your configuration";
        statusField.classList.value = "border-red form-status-field";
        statusField.style.visibility = "visible";
    });
}