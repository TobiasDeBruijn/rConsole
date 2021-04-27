import "../scss/index.scss";
import { loadConfig } from "./config";

export { setupLoginPage     } from "./login";
export { setupDashboard, loadControlButtons } from "./dashboard/dashboard";
export { loadFiles } from "./dashboard/files"; 
export { setupConsoleLoop   } from "./dashboard/console";
export { setupCharts        } from "./dashboard/charts";
export { setupFileEditor } from './dashboard/file-editor';

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