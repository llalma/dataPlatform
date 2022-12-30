<script lang="ts">
    import {onMount} from "svelte";
    import init, {Grid} from '../../../rustFunctions/grid/pkg/Grid';

    let data;
    let grid = [4,3];

    //Mount Rust WASM Function
    onMount(async () => {
        await init().then(() => {
            //Create Grid Datatype
            data = new Grid(grid[0], grid[1])
        })
    });

    function set_cell(e, i, j){
        data.set_cell(i, j, e.target.innerText)
        data=data
    }


</script>

<button on:click={console.log(data.to_csv())}>Save To CSV</button>

{#if data}

    <table style="table">
        <!--Insert data cells-->
        {#each Array(grid[0]) as _,i}
                <tr>

                    {#each Array(grid[1]) as _,j}
                        <td contenteditable="true"
                            on:blur={(event) => set_cell(event, i,j)}>
                            {data.get_cell(i,j)}
                        </td>

                    {/each}
                </tr>
        {/each}
    </table>

{/if}

<style>

    table{
        border-collapse: collapse;
    }

    tr{
        border: solid;
        border-width: 1px;
    }

    td{
        border: solid;
        border-width: 2px;
    }
</style>


