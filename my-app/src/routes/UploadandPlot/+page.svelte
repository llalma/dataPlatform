<script lang="ts">
    import {onMount} from 'svelte';
    //Import Grid component and set size of grid
    import Grid from './Grid.svelte'

    onMount(async () => {

    });

    //Write file parser in rust
    const onFileSelected = async (e)=>{
        // console.log(e.target.files[0])
        // await upload_excel(e.target)
        // await test()
    }
</script>



<!--<div>-->
<!--    &lt;!&ndash; File input &ndash;&gt;-->
<!--    <input type="file" accept=".csv, .xlsx" on:change={onFileSelected}>-->
<!--</div>-->

<div>
    <!--Display grid-->
    <Grid/>
</div>