<script lang="ts">
    import {onMount} from "svelte";
    import init, {Coordinate, Grid} from '../../../rustFunctions/grid/pkg/Grid';

    const visRowsDiff = 20
    const visColumnsDiff = 10

    let data, drag_start, drag_end
    let currentDragLoc;
    let dragBool = false;
    let keepHighlightedBool = false;

    let grid = [30, 10];
    let visRows = 0;
    let visColumns = 0;

    //For filtering
    let test = -1;

    let filter_column = ''
    let filter_values = []
    let checked_filters = []
    let previously_selected_filters = {}


    //For modal
    // let filter_column = ''
    // let filter_values = []
    // let checked_filters = []


    //Mount Rust WASM Function
    onMount(async () => {
        await init().then(() => {
            //Create Grid Datatype
            data = new Grid(grid[0], grid[1])
        })
    });

    function resize(){
        data.resize(grid[0], grid[1])
        data=data
    }

    function onHeaderUpdate(e, i){
        data.set_header(i, e.target.innerText)
    }

    function onCellUpdate(e, x, y){
        data.set_cell(new Coordinate(x, y), e.target.innerText)
        data=data
    }

    function getInfo(){
        console.log(data.height)
        console.log(data.width)
    }

    function start_drag(e, x, y){
        drag_start = new Coordinate(x, y)
        currentDragLoc = new Coordinate(x,y)
        dragBool = true

        if (drag_start.x !== currentDragLoc.x && drag_start.y !== currentDragLoc.y){
            keepHighlightedBool = false
        }

    }

    function mid_drag(e,x,y){
        if (dragBool){
            currentDragLoc = new Coordinate(x,y)
        }
    }

    function end_drag(e, x, y){
        drag_end = new Coordinate(x, y)

        if (drag_start.x !== currentDragLoc.x && drag_start.y !== currentDragLoc.y){
            keepHighlightedBool = true
        }

        //Only perform drag action if the drag goes to a different cell, just removes clicks
        if (drag_start.x !== drag_end.x && drag_start.y !== drag_end.y) {
            currentDragLoc = new Coordinate(-1,-1)
            console.log(data.get_dragged_cells(drag_start, drag_end))
        }

        dragBool = false
    }

    function copy(){
        navigator.clipboard.writeText(data.get_csv_string(drag_start, drag_end))
    }

    function paste(){
        navigator.clipboard.readText().then(res => {
            data.paste(currentDragLoc, res)
            data = data
        })
    }

    function to_csv(){
        const fileName = "test.csv"

        console.log(data.height)

        console.log(data.get_csv_export(new Coordinate(0,0), new Coordinate(data.width, data.height)))

        const blob = new Blob([data.get_csv_export(new Coordinate(0,0), new Coordinate(data.width, data.height))], {type: 'text/plain'});
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.download = fileName;
        link.href = url;
        link.click();
        URL.revokeObjectURL(url);
    }

    function delete_handle(){
        data.delete_area(drag_start, drag_end)
        data = data
        keepHighlightedBool = false
    }

    function upload_handle(e){
        const reader = new FileReader();
        reader.readAsText(e.target.files[0])
        reader.onload = () => {
            let txt = reader.result;
            console.log(txt);

            //Update the headers
            data.set_headers(txt.split("\n")[0])

            //Update the cell data
            data.paste(new Coordinate(0,0), txt.split("\n").slice(1).join("\n"))

            //Resize the grid to the needed size
            grid[0] = txt.split("\n").length
            grid[1] = txt.split("\n")[0].split(",").length
            resize()

        };
    }

    const table_scroll_handle = e => {

        if (e.shiftKey){
            //Horizontal scrolling - If shiftKey held down

            //Increment and decrement scroll by 1 row at a time
            visColumns += (e.deltaY > 0) ? (1) : (-1)

            //Handling for min and max values
            visColumns = (visColumns + visColumnsDiff > data.width) ? (data.width - visColumnsDiff) : (visColumns)
            visColumns = (visColumns < 0) ? (0) : (visColumns)

        } else {
            //Vertical Scrolling

            //Increment and decrement scroll by 1 row at a time
            visRows += (e.deltaY > 0) ? (1) : (-1)

            //Handling for min and max values
            visRows = (visRows + visRowsDiff > data.height) ? (data.height - visRowsDiff) : (visRows)
            visRows = (visRows < 0) ? (0) : (visRows)
        }
    };

</script>

<!--Adjust number of rows and columns in datatable
and show current postions being displated-->
<div>
    rows: <input type="number" min="0" bind:value={grid[0]} on:blur={resize(grid[0], grid[1])}>
    columns: <input type="number" min="0" bind:value={grid[1]} on:blur={resize(grid[0], grid[1])}>

    Displaying :
        Rows : {visRows}:{visRows+visRowsDiff}
        Columns : {visColumns}:{visColumns+visColumnsDiff}
</div>

<!--Div for buttons-->
<div>
    <button on:click={getInfo}>Get Info</button>
    <button on:click={to_csv}>To CSV</button>
    <button on:click={copy}>Copy</button>
    <button on:click={paste}>Paste</button>
    <button on:click={delete_handle}>Delete</button>
    <input type="file" accept=".csv" on:change={(e)=>upload_handle(e)}>
</div>

<div>
    <input type="number" bind:value={test}>
    <button on:click={filter}>filter</button>
</div>

<!--Statement to ensure display does not happened before data is laoded-->
{#if !data}
    Loading..
{:else}
    <table class="grid" on:wheel={table_scroll_handle}>

        <!--Insert Header rows and cells-->
        <tr>
            <th class="index-cell">Index</th>
            {#each Array.from(Array(Math.min(visColumns+visColumnsDiff, data.width)).keys()).slice(visColumns) as j}
                <th contenteditable="true" class="header" on:click={() => get_filter_values(data.get_header(j))} on:blur={onHeaderUpdate(event, j)}>{data.get_header(j)}</th>
            {/each}
        </tr>

        <!--Insert data cells-->
        {#each Array.from(Array(Math.min(visRows+visRowsDiff, data.height)).keys()).slice(visRows) as i}
            {#if data.get_visible(i)}
                <tr>
                    <!--Index Column-->
                    <td class="index-cell">{i}</td>

                {#each Array.from(Array(Math.min(visColumns+visColumnsDiff, data.width)).keys()).slice(visColumns) as j}
                    <td contenteditable="true" class="{((dragBool &&
                                            (drag_start.x !== currentDragLoc.x || drag_start.y !== currentDragLoc.y)
                                            && Math.min(drag_start.x, currentDragLoc.x)<=i && Math.max(currentDragLoc.x, drag_start.x)>=i
                                            && Math.min(drag_start.y, currentDragLoc.y)<=j && Math.max(currentDragLoc.y, drag_start.y)>=j)
                                            ||
                                            (keepHighlightedBool
                                            && Math.min(drag_start.x, drag_end.x)<=i && Math.max(drag_end.x, drag_start.x)>=i
                                            && Math.min(drag_start.y, drag_end.y)<=j && Math.max(drag_end.y, drag_start.y)>=j))



                                             ? 'highlight-cell' : 'cell'}" on:click={console.log(i)} on:blur={onCellUpdate(event, i,j)} on:mousedown={start_drag(event, i, j)} on:mouseup={end_drag(event, i, j)} on:mouseover={mid_drag(event, i, j)}>{data.get_cell(new Coordinate(i,j))}</td>

                {/each}
                </tr>
            {/if}
        {/each}
    </table>
{/if}

<style>
    /*Grid for excel*/
    .grid{
        display: table;
        width: 100%;
        border-spacing:0;
        border-collapse: collapse;
        border: 0.01px solid black;
    }

    .header{
        border: 5px solid rgba(0, 0, 0, 0.8);
        text-align: center;
    }

    /*Index cell*/
    .index-cell{
        border: 0.01px solid rgba(0, 0, 0, 0.8);
        text-align: center;
        width: 5px;
        border-collapse:collapse;
        border-right: 5px solid rgba(0, 0, 0, 0.8);
    }

    /*Individual cell for each grid*/
    .cell{
        border: 0.01px solid rgba(0, 0, 0, 0.8);
        text-align: center;
        width: 75px;
        border-collapse:collapse
    }

    .highlight-cell{
        border: 0.01px solid rgba(0, 0, 0, 0.8);
        text-align: center;
        background-color: red;
        width: 75px;
        border-collapse:collapse
    }

</style>