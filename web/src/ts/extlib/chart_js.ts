declare class Chart {
    data: IChartData;
    constructor(a: any, b: any);
    update(): void;
    addData(a: number): void;
}

declare interface IChartData {
    datasets: IChartDatasets[]
    labels: number[]
}

declare interface IChartDatasets {
    data: number[]
}

export default Chart;