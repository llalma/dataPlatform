<script>
    import { onMount } from 'svelte';
    import init, {Chart} from '../../../rust/pkg/wasm_game_of_life';

    function updatePlot() {
        chartStatus = `Rendering Power: ${plotPower}...`;
        chart = Chart.power("canvas", plotPower, shift_x, shift_y, zoomLevel, plottingFactor)        
        chartStatus = `Rendered Power: ${plotPower}`;
    } 

    const pan_factor = 100

    let canvas;
    let mouse = {x:0, y:0}
    let chartStatus = ''
    let plotPower = 3
    let plottingFactor = 50
    let shift_x = 0;
    let shift_y = 0;
    let chart = null;

    let zoomLevel = 0;

    //Panning functionality
    let mouseClicked = false;
    let pan= {x:0, y:0};
    let pan_origin = {x:0, y:0};


    const canvasDim = {
        width: 800,
        height: 800
    };

    const handleMouseMovement = (e) => {
        if (mouseClicked){
            pan.x = e.pageX - pan_origin.x;
            pan.y = e.pageY - pan_origin.y;
            
            shift_x = pan.x/pan_factor;
            shift_y = pan.y/pan_factor;
            updatePlot()
        }
    }

    const startDrag = (e) => {
        pan_origin.x = shift_x*pan_factor
        pan_origin.y = shift_y*pan_factor
        mouseClicked = true
    }

    const endDrag = (e) => {
        mouseClicked = false
    }

    const handleScroll = (e) => {       
        if(e.wheelDelta > 0){
            zoomLevel++
        }else{
            zoomLevel--
        }

        //Redraw graph
        //Probbably shouldnt be redrawing for every scroll
        updatePlot()
    }

    onMount(async () => {
        init().then(() => {
            chart = Chart.power("canvas", plotPower, shift_x, shift_y, zoomLevel, plottingFactor)        
        })
    });


</script>

<div>
    x: {mouse.x} y: {mouse.y}
</div>

<div><input type=number bind:value={plotPower} on:change={updatePlot}></div>
<div>shift_x<input type=number bind:value={shift_x} on:change={updatePlot}></div>
<div>shift_y<input type=number bind:value={shift_y} on:change={updatePlot}></div>
<div><input type=number bind:value={plottingFactor} on:change={updatePlot}></div>

<div>
    zoomLevel: {zoomLevel}
</div>

<div>
    {chartStatus}
</div>
   
<canvas id='canvas' on:wheel={handleScroll} on:mousemove={handleMouseMovement} on:mousedown={startDrag} on:mouseup={endDrag} bind:this={canvas} width={canvasDim.width} height={canvasDim.height} style="border:2px solid black !important" />
