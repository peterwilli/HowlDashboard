<script>
    import { Line } from 'svelte-chartjs'
    import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale } from 'chart.js';

    ChartJS.register(Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale);

    export let data = {};

    function howlToChartJS(howlData) {
        let result = {
            labels: howlData.xPoints,
            datasets: []
        };
        result.datasets.push({
            label: howlData.title,
            data: howlData.values[0],
            fill: true,
            lineTension: 0.3,
            backgroundColor: 'rgba(184, 185, 210, .3)',
            borderColor: 'rgb(35, 26, 136)',
            borderCapStyle: 'butt',
            borderDash: [],
            borderDashOffset: 0.0,
            borderJoinStyle: 'miter',
            pointBorderColor: 'rgb(35, 26, 136)',
            pointBackgroundColor: 'rgb(255, 255, 255)',
            pointBorderWidth: 10,
            pointHoverRadius: 5,
            pointHoverBackgroundColor: 'rgb(0, 0, 0)',
            pointHoverBorderColor: 'rgba(220, 220, 220, 1)',
            pointHoverBorderWidth: 2,
            pointRadius: 1,
            pointHitRadius: 10
        })
        console.log("result", JSON.stringify(result, null, 2));
        return result;
    }

    $: convertedData = howlToChartJS(data);
</script>

<div>
    <Line data={convertedData} />
</div>