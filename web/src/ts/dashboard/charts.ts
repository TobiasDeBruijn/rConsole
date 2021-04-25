import Chart from "../extlib/chart_js";
import { METRIC_ENDPOINT } from '../config';
import { setup } from '../index';
import { IMetricResponse } from '../server_types';
import { getCookie } from '../util';

const GRAPH_POINTS: number = 40;

const CPU_CHART_CONFIGURATION = {
    type: 'line',
    data: {
        datasets: [
            {
                label: "% CPU Usage",
                data: [0],
                borderColor: 'rgb(29, 102, 181)',
                backgroundColor: 'rgba(29, 102, 181, 0.5)',
                fill: true,
                tension: 0.4
            }
        ]
    },
    options: {
        responsive: true,
        plugins: {
            title: {
                display: false
            },
            legend: {
                display: false
            }
        },
        scales: {
            x: {
                display: true,
                title: {
                    display: true,
                    text: "Time",
                    color: "#CAD1D9",
                    font: {
                        family: "'Noto Sans', sans-serif",
                        weight: 400
                    }
                },
                //Disable grid lines
                grid: {
                    display: false
                },
                //We don't want to show the Unix timestamps
                ticks: {
                    display: false
                }
            },
            y: {
                display: true,
                title: {
                    display: true,
                    color: "#CAD1D9",
                    font: {
                        family: "'Noto Sans', sans-serif",
                        weight: 400
                    },
                    text: '% CPU Usage'
                },
                //Disable grid lines
                grid: {
                    display: false
                },
                suggestedMin: 0,
                ticks: {
                    color: "#CAD1D9",
                    font: {
                        family: "'Noto Sans', sans-serif",
                        weight: 400
                    }
                }
            }
        }
    }
}

const MEM_CHART_CONFIGURATION = {
    type: 'line',
    data: {
        datasets: [
            {
                data: [0],
                label: "Memory Usage (MB)",
                borderColor: 'rgb(29, 102, 181)',
                backgroundColor: 'rgba(29, 102, 181, 0.5)',
                fill: true,
                tension: 0.4
            }
        ]
    },
    options: {
        responsive: true,
        plugins: {
            title: {
                display: false
            },
            legend: {
                display: false
            }
        },
        scales: {
            x: {
                display: true,
                title: {
                    display: true,
                    text: "Time",
                    color: "#CAD1D9",
                    font: {
                        family: "'Noto Sans', sans-serif",
                        weight: 400
                    }
                },
                //Disable grid lines
                grid: {
                    display: false
                },
                //We don't want to show the Unix timestamps
                ticks: {
                    display: false
                }
            },
            y: {
                display: true,
                title: {
                    display: true,
                    text: 'Memory Usage (MB)',
                    color: "#CAD1D9",
                    font: {
                        family: "'Noto Sans', sans-serif",
                        weight: 400
                    }
                },
                //Disable grid lines
                grid: {
                    display: false
                },
                suggestedMin: 0,
                ticks: {
                    color: "#CAD1D9",
                    font: {
                        family: "'Noto Sans', sans-serif",
                        weight: 400
                    }
                }
            }
        }
    }
}

export async function setupCharts() {
    await setup();

    let cpuChartCtx = <HTMLCanvasElement> document.getElementById("cpu-graph");
    let cpuChart = new Chart(cpuChartCtx, CPU_CHART_CONFIGURATION);

    let memChartCtx = <HTMLCanvasElement> document.getElementById("mem-graph");
    let memChart = new Chart(memChartCtx, MEM_CHART_CONFIGURATION);

    getMetricsData().then((data) => {
        if(data.status != 200) {
            return;
        }

        //Sort the data by the epoch
        data.metrics.sort((a, b) => (a.epoch > b.epoch) ? 1 : -1);
        
        updateMemChart(memChart, data);
        updateCpuChart(cpuChart, data);
    });

    let chartUpdateInterval = window.setInterval(async () => {
        let metricsResponse = await getMetricsData();
        if(metricsResponse.status != 200) {
            window.clearInterval(chartUpdateInterval);
            return;
        }
            
        //Sort the data by the epoch
        metricsResponse.metrics.sort((a, b) => (a.epoch > b.epoch) ? 1 : -1);
        
        //Update the charts
        updateMemChart(memChart, metricsResponse);
        updateCpuChart(cpuChart, metricsResponse);
    }, 10_000);

    cpuChartCtx.style.visibility = 'visible';
}

/**
 * Update the memory chart with the new data
 * @param memChart The memory chart to update
 * @param data The new data to insert
 */
async function updateMemChart(memChart: Chart, data: IMetricResponse) {

    //Insert the data into the chart
    data.metrics.forEach((metricsEntry) => {
        if(memChart.data.labels.includes(metricsEntry.epoch)) {
            return;
        }

        if(memChart.data.labels.length > GRAPH_POINTS) {
            memChart.data.labels.shift();
        }

        memChart.data.labels.push(metricsEntry.epoch);

        memChart.data.datasets.forEach((dataset) => {
            if(dataset.data.length > GRAPH_POINTS) {
                dataset.data.shift();
            }

            dataset.data.push((metricsEntry.mem.total_mem - metricsEntry.mem.free_mem))
        });
    });

    memChart.update();
}

/**
 * Update the CPU chart with the new data
 * @param cpuChart The CPU Chart to update
 * @param data The new data to insert
 */
async function updateCpuChart(cpuChart: Chart, data: IMetricResponse) {
    //Insert the data into the chart
    data.metrics.forEach((metricEntry) => {
        if(cpuChart.data.labels.includes(metricEntry.epoch)) {
            return;
        }

        //Remove the 1st label, and add the new one
        if(cpuChart.data.labels.length > GRAPH_POINTS) {
            cpuChart.data.labels.shift();
        }

        cpuChart.data.labels.push(metricEntry.epoch);

        cpuChart.data.datasets.forEach((dataset) => {
            //Remove the first data entry, and add the new one
            if(dataset.data.length > GRAPH_POINTS) {
                dataset.data.shift();
            }
            // multiply by 100, because we get the data as a fraction, and we want it as a percentage
            dataset.data.push(metricEntry.cpu.load_avg * 100);

        });
    });
    cpuChart.update();
}

/**
 * Fetch Metrics data from the backend server
 * @returns The server's response
 */
async function getMetricsData(): Promise<IMetricResponse> {
    let getMetricRequest = $.ajax({
        url: METRIC_ENDPOINT,
        method: 'POST',
        data: {
            session_id: getCookie("sessionid")
        }
    });

    return <IMetricResponse> await getMetricRequest;
}