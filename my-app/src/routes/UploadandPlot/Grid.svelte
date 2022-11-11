<script lang="ts">
    import {onMount} from "svelte";
    import init, {Grid, Coordinate} from '../../../RustFunctions/Grid/pkg/Grid';

    let data, drag_start, drag_end
    let currentDragLoc;
    let dragBool = false;
    let keepHighlightedBool = false;


    let grid = [10, 10];

    //Mount Rust WASM Function
    onMount(async () => {
        await init().then(() => {
            //Create Grid Datatype
            data = new Grid(grid[0], grid[1])
        })
    });


    //On header update
    function onHeaderUpdate(e, i){
        data.set_header(i, e.target.value)
    }

    //On cell update
    function onCellUpdate(e, x, y){
        data.set_cell(new Coordinate(x, y), e.target.value)
    }

    //Get cell
    function getInfo(x, y){
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

    function to_csv(){
        const fileName = "test.csv"

        const blob = new Blob([data.to_csv()], {type: 'text/plain'});
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.download = fileName;
        link.href = url;
        link.click();
        URL.revokeObjectURL(url); // Object URLs should be revoked after use
    }

    function delete_handle(){
        console.log(drag_start)
        console.log(drag_end)
        data.delete_area(drag_start, drag_end)
    }
</script>

<!--Adjust number of rows and columns in datatable-->
<div>
    rows: <input type="number" bind:value={grid[0]} on:change={data.update_height_width(grid[0], grid[1])}>
    columns: <input type="number" bind:value={grid[1]} on:change={data.update_height_width(grid[0], grid[1])}>
</div>

<!--Div for buttons-->
<div>
    <button on:click={getInfo}>Get Info</button>
    <button on:click={to_csv}>To CSV</button>
    <button on:click={delete_handle}>Delete</button>
</div>

<!--Create grid div-->
<div class="grid" style="grid-template-rows: repeat({grid[0]}, auto); grid-template-columns:repeat({grid[1]}, auto)">
    <!--    Display each header in the grid-->
    {#each {length: grid[1]} as _, i (i)}
        <input type="text" class="header" on:change={()=>onHeaderUpdate(event, i)}>
    {/each}

    <!--    Display each cell in the grid-->
    {#each {length: grid[0]} as _, i (i)}
        {#each {length: grid[1]} as _, j (j)}
            <input type="text" class="{(dragBool &&
                                        (drag_start.x !== currentDragLoc.x || drag_start.y !== currentDragLoc.y)
                                        && Math.min(drag_start.x, currentDragLoc.x)<=i && Math.max(currentDragLoc.x, drag_start.x)>=i
                                        && Math.min(drag_start.y, currentDragLoc.y)<=j && Math.max(currentDragLoc.y, drag_start.y)>=j)

                                        ||
                                        (keepHighlightedBool
                                        && Math.min(drag_start.x, drag_end.x)<=i && Math.max(drag_end.x, drag_start.x)>=i
                                        && Math.min(drag_start.y, drag_end.y)<=j && Math.max(drag_end.y, drag_start.y)>=j)

                                         ? 'highlight-cell' : 'cell'}" on:click={()=>console.log(data.get_cell(new Coordinate(i,j)))} on:change={()=>onCellUpdate(event, i,j)} on:mousedown={start_drag(event, i, j)} on:mouseup={end_drag(event, i, j)} on:mouseover={mid_drag(event, i, j)}>
        {/each}
    {/each}
</div>


<style>
    /*Grid for excel*/
    .grid{
        display: grid;
    }

    .header{
        border: 5px solid rgba(0, 0, 0, 0.8);
        text-align: center;
    }

    /*Individual cell for each grid*/
    .cell{
        border: 1px solid rgba(0, 0, 0, 0.8);
        text-align: center;
    }

    .highlight-cell{
        border: 1px solid rgba(0, 0, 0, 0.8);
        text-align: center;
        background-color: red;
    }

</style>