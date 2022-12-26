<script>
	import { Client } from "libhowl";
	import { onMount } from "svelte";
    import Chart from "@/DashboardComponents/Chart.svelte";

	let data = {};

	onMount(function() {
		if(window.location.hash.length == 0) {
			window.location.hash = prompt("Enter a howl server to connect to!");
		}
		let server = window.location.hash.slice(1);
		const subscriber = Client.subscriber();
		subscriber.connect(server);
		const token = subscriber.listenForData(function(json) {
			console.log("listenForData cb -> ", json);
			data = Object.assign(data, json);
		});
		console.log("token", token);
	})
</script>

<main>
	<div style="height: 300px;">
		{#each Object.entries(data) as [key, value] (key)}
			{#if value.dataType === "Chart"}
			<Chart data={value}></Chart>
			{/if}
		{/each}
	</div>
</main>
