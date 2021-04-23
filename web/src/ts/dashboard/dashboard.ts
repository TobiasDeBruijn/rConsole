import { isLoggedIn, doLogout } from "../auth";
import { setup                } from "../index";

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
    document.getElementById("githubBtn").addEventListener("click", (_e) => { /*window.location.href = "https://github.com/TheDutchMC/rConsole"*/ window.open("https://github.com/TheDutchMC/rConsole", "_blank"); });
    document.getElementById("logoutBtn").addEventListener("click", (_e) => { doLogout() });
}