import * as $ from 'jquery';

interface IConfig {
    uri:    string
}

export async function loadConfig() {
    let getConfigRequest = $.ajax({
        url: "/rconsole_web_config.json",
        method: "GET"
    });

    let config = <IConfig> await getConfigRequest;

    LOGIN_ENDPOINT =            config.uri + "/auth/login";
    SESSION_ENDPOINT =          config.uri + "/auth/session";

    CONSOLE_NEW_ENDPOINT =      config.uri + "/console/new";
    CONSOLE_ALL_ENDPOINT =      config.uri + "/console/all";
    CONSOLE_COMMAND_ENDPOINT =  config.uri + "/console/command";

    METRIC_ENDPOINT =           config.uri + "/stats/metrics";
}

export let LOGIN_ENDPOINT:              string;
export let SESSION_ENDPOINT:            string;

export let CONSOLE_ALL_ENDPOINT:        string;
export let CONSOLE_NEW_ENDPOINT:        string;
export let CONSOLE_COMMAND_ENDPOINT:    string;

export let METRIC_ENDPOINT:             string;