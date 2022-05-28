<div class="counter">
    {#if total != null}
    <div class="total">
        <div class="entry">
            <div class="border">
                <div class="inner">
                    <div class="big-value">
                        <div class="number">{total.number}</div>
                        {#if 'suffix' in total}
                            <div class="suffix">
                                {total.suffix}
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div class="line-break"></div>
    {/if}
    <div class="sub-entries">
        {#each Object.entries(data) as [key, value] (key)}
            <div class="entry">
                <div class="border">
                    <div class="inner">
                        {#if value.length == 2}
                            <div class="big-value">
                                {#if 'number' in value[0]}
                                <div class="number">
                                    {value[0].number}
                                    </div>
                                {/if}
                                {#if 'suffix' in value[0]}
                                <div class="suffix">
                                    {value[0].suffix}
                                    </div>
                                {/if}
                            </div>
                            <div class="small-value">
                                {#if 'number' in value[1]}
                                <div class="number">
                                    {value[1].number}
                                    </div>
                                {/if}
                                {#if 'suffix' in value[1]}
                                <div class="suffix">
                                    {value[1].suffix}
                                    </div>
                                {/if}
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        {/each}
    </div>
</div>

<script>
    import { onMount } from "svelte";
    export let data;
    let total = null;

    function calculatePossibleTotal() {
        // In case there's a single on all records, we can use it as total.
        let suffixes = {}
        for(let key in data) {
            for(let number of data[key]) {
                if(number.suffix in suffixes) {
                    suffixes[number.suffix]++
                }
                else {
                    suffixes[number.suffix] = 1
                }
            }
        }
        const dataLength = Object.keys(data).length
        for(let suffix in suffixes) {
            if(suffixes[suffix] === dataLength) {
                // We have one suffix across all data
                total = {
                    number: 0,
                    suffix
                }
                for(let key in data) {
                    for(let number of data[key]) {
                        if(number.suffix == suffix) {
                            total.number += parseFloat(number.number)
                        }
                    }
                }
                break
            }
        }
    }

    onMount(function() {
        calculatePossibleTotal()
    })
</script>