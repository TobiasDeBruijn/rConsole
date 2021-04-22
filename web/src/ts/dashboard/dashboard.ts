import { isLoggedIn, doLogout } from "../auth";
import { setup                } from "../index";

export async function setupDashboard() {
    await setup();

    //Check authentication
    if(!await isLoggedIn()) {
        window.location.href = "../login.html";
        return;
    }

    loadButtons();
}

export function loadButtons() {
    document.getElementById("githubBtn").addEventListener("click", (_e) => { window.location.href = "https://github.com/TheDutchMC/rConsole" });
    document.getElementById("logoutBtn").addEventListener("click", (_e) => { doLogout() });
}