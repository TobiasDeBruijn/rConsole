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
    message:        string;
    timestamp:      number;
    level:          string;
    thread:         string;
    log_attributes: LogAttribute[];
}

export enum LogAttribute {
    LogNoIndex
}

export interface IMetricResponse {
    status:         number;
    metrics:        IMetricEntry[];
}

export interface IMetricEntry {
    epoch:          number;
    cpu:            ICpuMetric;
    mem:            IMemMetric;
    player:         number
}

export interface ICpuMetric {
    load_avg:       number;
}

export interface IMemMetric {
    max_mem:        number;
    total_mem:      number;
    free_mem:       number;
}