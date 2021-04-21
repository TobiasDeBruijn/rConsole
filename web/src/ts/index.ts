import "../scss/index.scss";
import { loadConfig } from "./config";

export { setupLoginPage     } from "./login";
export { setupDashboard     } from "./dashboard/dashboard";
export { setupConsoleLoop   } from "./dashboard/console";

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

