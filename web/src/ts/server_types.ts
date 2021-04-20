export interface ILoginResponse {
    status:     number;
    session_id: string;
}

export interface ISessionResponse {
    status:     number;
}

export interface ILogResponse {
    status:     number;
    logs:       ICombinedLogEntry[];
}

export interface ICombinedLogEntry {
    id:         number;
    log_entry:  ILogEntry;
}

export interface ILogEntry {
    message:    string;
    timestamp:  number;
    level:      string;
    thread:     string;
}