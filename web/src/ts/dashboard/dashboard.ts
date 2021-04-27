import { isLoggedIn, doLogout } from "../auth";
import { CONSOLE_COMMAND_ENDPOINT } from "../config";
import { setup                } from "../index";
import { getCookie } from "../util";

export async function setupDashboard() {
    await setup();
    loadNavButtons();

    //Check authentication
    if(!await isLoggedIn()) {
        window.location.href = "../login.html";
        return;
    }
}

function loadNavButtons() {
    //navigatrion
    document.getElementById("githubBtn").addEventListener("click", (_e) => { window.open("https://github.com/TheDutchMC/rConsole", "_blank"); });
    document.getElementById("logoutBtn").addEventListener("click", (_e) => { doLogout() });
    document.getElementById("consoleBtn").addEventListener("click", (_e) => window.location.href = "index.html");
    document.getElementById("filesBtn").addEventListener("click", (_e) => window.location.href = "files.html");
}

export function loadControlButtons() {
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