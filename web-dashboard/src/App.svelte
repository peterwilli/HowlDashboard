<script>
	import Counter from "@/DashboardComponents/Counter.svelte";
	import { Client } from "libhowl";
	import { onMount } from "svelte";

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
	})
</script>

<main>
	{#each Object.entries(data) as [key, value] (key)}
		{#if key == "categorical_number_data"}
			{#each Object.entries(value) as [title, value2] (title)}
				<h3>{title}</h3>
				<Counter data={value2} />
			{/each}
		{/if}
	{/each}
</main>
