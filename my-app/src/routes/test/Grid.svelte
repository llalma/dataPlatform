<script lang="ts">
    import {onMount} from "svelte";
    import init, {Grid} from '../../../rustFunctions/grid/pkg/Grid';

    let data;
    let grid = [4,3];

    //File uploaded to load data
    let upload_file;

    //Mount Rust WASM Function
    onMount(async () => {

        await init().then(() => {
            //Create Grid Datatype
            data = new Grid(grid[0], grid[1])
        });
        // await initThreadPool();
    });

    function set_cell(e, i, j){
        data.set_cell(i, j, e.target.innerText)
        data=data
    }

    function download_csv_file(){
        const fileName = "test.csv"

        const blob = new Blob([data.to_csv()], {type: 'text/plain'});
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.download = fileName;
        link.href = url;
        link.click();
        URL.revokeObjectURL(url);
    }

    //TODO : This is assumiong first col is index and top row is a header. NEEDS to be Better.
    function load_csv_file(event){


        const file = event.target[0].files[0]

        // console.log(data.load_csv(file))

        let reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onload = data1 => {
            console.log(data1.target.result)

            console.log(data.load_csv(new Uint8Array(data1.target.result)))
        };
    }


</script>

<!--Load CSV File-->
<form on:submit|preventDefault={() => load_csv_file(event)}>
    <label>File</label>
    <input type="file" accept="text/csv" bind:value={upload_file}>
    <button type="submit">Load File</button>
</form>

<button on:click={download_csv_file}>Save To CSV</button>

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


