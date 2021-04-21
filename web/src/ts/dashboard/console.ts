import { CONSOLE_ALL_ENDPOINT, CONSOLE_NEW_ENDPOINT } from '../config';
import { ILogResponse                               } from '../server_types';
import { getCookie                                  } from '../util';

export async function setupConsoleLoop() {
    let consoleAll = getConsole(false);
    consoleAll.then((e) => {

    });

    window.setInterval(() => {
        let consoleNew = getConsole();
        consoleNew.then((e) => {
            let parseSuccessful=  parseConsoleResponse(e);
            if(!parseSuccessful) {
                window.clearInterval(this);
                console.error("An error occurred while parsing the log responses.");
            }
        });
    }, 1000);
}

async function getConsole(onlyNew = true): Promise<ILogResponse> {
    let getConsoleRequest = $.ajax({
        url: (onlyNew) ? CONSOLE_NEW_ENDPOINT : CONSOLE_ALL_ENDPOINT,
        method: 'POST',
        data: {
            session_id: getCookie('sessionid')
        }
    });

    let getConsoleResponse = <ILogResponse> await getConsoleRequest;

    return getConsoleResponse;
}

function parseConsoleResponse(response: ILogResponse, emptyConsole = false): boolean {
    if(response.status != 200) {
        return false;
    }

    const CONSOLE_VIEW = document.getElementById("console-view");
    if(emptyConsole) {
        for(let i = 0; i < CONSOLE_VIEW.children.length; i++) {
            let child = CONSOLE_VIEW.children[i];
            CONSOLE_VIEW.removeChild(child);
        }
    }

    response.logs.forEach((e) => {
        let logEntry = document.createElement("div");
        logEntry.id = e.id.toString();
        //TODO Timestamp from Epoch to [HH:mm:ss]
        logEntry.innerHTML = "[" + e.log_entry.thread + "/" + e.log_entry.level + "] " + e.log_entry.message;
        
        CONSOLE_VIEW.appendChild(logEntry);
    });

    return true;
}