<script>
	import { onMount } from "svelte";
	export let data;
	let sortedKeys = [];
	let total = null;

	function calculatePossibleTotal() {
		// In case there's a single on all records, we can use it as total.
		let suffixes = {};
		for (let key in data) {
			const entry = data[key]
			if ('converted_values' in entry) {
				for(let converted_key in entry.converted_values) {
					if (converted_key in suffixes) {
						suffixes[converted_key]++;
					} else {
						suffixes[converted_key] = 1;
					}
				}
			}
		}
		const dataLength = Object.keys(data).length;
		for (let suffix in suffixes) {
			if (suffixes[suffix] === dataLength) {
				console.log(`Suffix ${suffix} can be used to calculate total!`);
				// We have one suffix across all data
				total = {
					number: 0,
					suffix,
				};
				for (let key in data) {
					const entry = data[key]
					if ('converted_values' in entry) {
						for(let converted_key in entry.converted_values) {
							if(converted_key == suffix) {
								total.number += parseFloat(entry.converted_values[converted_key]);
							}
						}
					}
				}
				break;
			}
		}
	}

	function sortedEntries(data) {
		let result = Object.entries(data);
		result = result.sort((a, b) => {
			a = a[0];
			b = b[0];
			if (a < b) {
				return -1;
			}
			if (a > b) {
				return 1;
			}
			return 0;
		});
		return result;
	}

	onMount(function () {
		calculatePossibleTotal()
	});
</script>

<div class="counter">
	{#if total != null}
		<div class="total">
			<div class="entry">
				<div class="border">
					<div class="inner">
						<div class="big-value">
							<div class="number">{parseFloat(total.number).toFixed(2)}</div>
							{#if "suffix" in total}
								<div class="suffix">
									Total {total.suffix}
								</div>
							{/if}
						</div>
					</div>
				</div>
			</div>
		</div>
		<div class="line-break" />
	{/if}
	<div class="sub-entries">
		{#each sortedEntries(data) as [name, value] (name)}
			<div class="entry">
				<div class="border">
					<div class="inner">
						<div class="big-value">
							<div class="number">
								{parseFloat(value.number).toFixed(2)}
							</div>
							{#if "suffix" in value}
								<div class="suffix">
									{value.suffix}
								</div>
							{/if}
						</div>
						{#each Object.entries(value.converted_values) as [converted_name, converted_value] (converted_name)}
							<div class="small-value">
								<div class="number">
									{parseFloat(converted_value).toFixed(2)}
								</div>
								<div class="suffix">
									{converted_name}
								</div>
							</div>
						{/each}
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
