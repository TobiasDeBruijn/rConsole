import "../scss/index.scss";

import { loadConfig, CONSOLE_NEW_ENDPOINT } from "./config";

export { setupLoginPage } from "./login";

export async function showDefaultHidden() {
    let allDefaultHidden = document.querySelectorAll("[data-default-hidden]");
    allDefaultHidden.forEach((e: HTMLElement) => {
        e.style.visibility = "visible";
    });
}

export async function setup() {
    showDefaultHidden();
    await loadConfig();   
}

