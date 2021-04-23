import AnsiUp from '../extlib/ansi_up';

import { CONSOLE_ALL_ENDPOINT, CONSOLE_COMMAND_ENDPOINT, CONSOLE_NEW_ENDPOINT } from '../config';
import { setup                                      } from '../index';
import { ILogResponse, LogAttribute                 } from '../server_types';
import { getCookie                                  } from '../util';

const ANSI_UP = new AnsiUp();

/**
 * Flag indicating if the last server response was an error.
 * If this is true we will no longer attempt to fetch new logs from the server
 * The user should refresh to clear this.
 */
let SERVER_IS_ERR = false;

export async function setupConsoleLoop() {
    await setup();
    const CONSOLE_VIEW = document.getElementById("console-view");

    let commandInputField = document.getElementById("console-input");
    commandInputField.addEventListener("keypress", (e) => {
        if(e.key == 'Enter') {
            handleCommandInput();
        }
    })

    let consoleAll = getLogs(false);
    consoleAll.then((e) => {
        parseConsoleResponse(e);
        CONSOLE_VIEW.scrollTo(0, CONSOLE_VIEW.scrollHeight);
    });

    let fetchLogsInterval = window.setInterval(() => {
        if(SERVER_IS_ERR) {
            window.clearInterval(fetchLogsInterval);
            //TODO add an error box on the frontend
            return;
        }

        let consoleNew = getLogs();
        consoleNew.then((e) => {
            //If the user has scrolled up less than 10%, we will scroll them to the bottom
            let shouldScroll = (CONSOLE_VIEW.scrollTop/CONSOLE_VIEW.scrollHeight >= 1 - 0.1);

            let parseSuccessful = parseConsoleResponse(e);
            if(!parseSuccessful) {
                console.error("An error occurred while parsing the log responses.");
                SERVER_IS_ERR = true;
                window.clearInterval(fetchLogsInterval)
                //TODO add an error box on the frontend
                return;
            }

            if(shouldScroll) {
                CONSOLE_VIEW.scrollTo(0, CONSOLE_VIEW.scrollHeight);
            }
        });
    }, 500);
}

async function handleCommandInput() {
    const COMMAND_INPUT = <HTMLInputElement> document.getElementById("console-input");
    const CONSOLE_VIEW = document.getElementById('console-view');

    let noLogEntry = document.createElement('div');
    noLogEntry.classList.value = "LOG_NO_INDEX";
    noLogEntry.innerHTML = ">" + COMMAND_INPUT.value;

    CONSOLE_VIEW.appendChild(noLogEntry);

    let execCommandReq = $.ajax({
        url: CONSOLE_COMMAND_ENDPOINT,
        method: 'POST',
        data: {
            session_id: getCookie('sessionid'),
            command: COMMAND_INPUT.value
        }
    });

    execCommandReq.fail((e) => {
        console.error(e);
    });

    execCommandReq.done((e) => {
        COMMAND_INPUT.value = "";
    });
}

/**
 * Fetch the logs from the server
 * @param onlyNew   Should only new LogEntry's be fetched (default: true)
 * @returns         Returns a Promise containing the server's response
 */
async function getLogs(onlyNew = true): Promise<ILogResponse> {    
    let getConsoleRequest = $.ajax({
        url: (onlyNew) ? CONSOLE_NEW_ENDPOINT : CONSOLE_ALL_ENDPOINT,
        method: 'POST',
        data: {
            session_id: getCookie('sessionid'),
            since: (onlyNew) ? getLastLogEntryId() : null
        }
    });

    getConsoleRequest.fail((e) => {
        console.error(e);
        SERVER_IS_ERR = true;
    })

    let getConsoleResponse = <ILogResponse> await getConsoleRequest;
    return getConsoleResponse;
}

/**
 * Get the ID of the last LogEntry in the CONSOLE_VIEW
 * @returns Returns the ID, or null if the console is empty
 */
function getLastLogEntryId(): string {
    const CONSOLE_VIEW = document.getElementById("console-view");

    //Determine the ID of the last LogEntry in the console
    //Some elements in the CONSOLE_VIEW do not have an ID,
    //These are classified as LOG_NO_INDEX and has that as a class.
    //An example of this is an executed command, e.g: '>help'
    let lastLogEntryId: string = null;
    for(let i = CONSOLE_VIEW.children.length -1; i > 0; i--) {
        let consoleViewEntry = CONSOLE_VIEW.children[i];
        if(consoleViewEntry.classList.contains("LOG_NO_INDEX")) {
            continue;
        }

        lastLogEntryId = consoleViewEntry.id;
        break;
    }

    return lastLogEntryId;
}

/**
 * Parse a response from the server and add it to the DOM where necessary
 * @param response      The response from the server
 * @param emptyConsole  Should the CONSOLE_VIEW be cleared before new elements are added (default: false)
 * @returns             Returns true if everything went OK
 */
function parseConsoleResponse(response: ILogResponse, emptyConsole = false): boolean {
    if(response.status != 200) {
        return false;
    }

    //The request was successful, but there were no new log entries
    if(response.status == 200 && response.logs == null) {
        return true;
    } 

    const CONSOLE_VIEW = document.getElementById("console-view");

    //If the emptyConsole flag is true, we need to remove all of CONSOLE_VIEW's children
    if(emptyConsole) {
        for(let i = 0; i < CONSOLE_VIEW.children.length; i++) {
            let child = CONSOLE_VIEW.children[i];
            CONSOLE_VIEW.removeChild(child);
        }
    }

    //Determine the ID of the last LogEntry in the console
    let lastLogEntryId = getLastLogEntryId();

    //If the last LogEntry in the server's response has the same ID as
    //the last LogEntry in CONSOLE_VIEW, we can return because we're up-to-date
    //with the server's logs 
    if(response.logs[response.logs.length -1].id.toString() == lastLogEntryId) {
        return true;
    } 

    //Iterate over every LogEntry returned from the server,
    response.logs.forEach((e) => {
        if(e.id.toString() == lastLogEntryId) return;
        let logEntry = document.createElement("div");

        //If the returned LogEntry has the LogNoIndex attribute,
        //we don't want to set an ID, but rather add the LOG_NO_INDEX class
        if(e.log_entry.log_attributes.includes(LogAttribute.LogNoIndex)) {
            logEntry.classList.add("LOG_NO_INDEX");
            logEntry.innerHTML = ANSI_UP.ansi_to_html(e.log_entry.message);
        } else {
            logEntry.id = e.id.toString();
        }

        //TODO Timestamp from Epoch to [HH:mm:ss]
        let date = new Date(e.log_entry.timestamp);
        let timestampFormatted = "[" + date.getHours() + ":" + date.getMinutes() + ":" + date.getSeconds() + "]"
        logEntry.innerHTML = timestampFormatted + " [" + e.log_entry.thread + "/" + e.log_entry.level + "]: " + ANSI_UP.ansi_to_html(e.log_entry.message);
        
        CONSOLE_VIEW.appendChild(logEntry);
    });

    return true;
}