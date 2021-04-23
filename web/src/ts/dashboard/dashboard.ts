import { isLoggedIn, doLogout } from "../auth";
import { CONSOLE_COMMAND_ENDPOINT } from "../config";
import { setup                } from "../index";
import { getCookie } from "../util";

export async function setupDashboard() {
    await setup();
    loadButtons();

    //Check authentication
    if(!await isLoggedIn()) {
        window.location.href = "../login.html";
        return;
    }
}

export function loadButtons() {
    //navigatrion
    document.getElementById("githubBtn").addEventListener("click", (_e) => { /*window.location.href = "https://github.com/TheDutchMC/rConsole"*/ window.open("https://github.com/TheDutchMC/rConsole", "_blank"); });
    document.getElementById("logoutBtn").addEventListener("click", (_e) => { doLogout() });

    //Server control
    document.getElementById('stop-server-btn').addEventListener("click", (_e) => {
        $.ajax({
           url: CONSOLE_COMMAND_ENDPOINT,
           method: 'POST',
           data: {
               session_id: getCookie('sessionid'),
               command: 'stop'
           } 
        });
    });
}