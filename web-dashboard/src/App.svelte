<script>
	import Counter from "@/DashboardComponents/Counter.svelte";
	let data = {
		categorical_number_data: {
			"test": [
				{ number: "120", suffix: "BTC" },
				{ number: "12990", suffix: "USD" },
			],
			"test2": [
				{ number: "110", suffix: "ETH" },
				{ number: "12990", suffix: "USD" },
			],
		},
	};
</script>

<main>
	{#each Object.entries(data) as [key, value] (key)}
		{#if key == "categorical_number_data"}
			<Counter data={value} />
		{/if}
	{/each}
</main>
