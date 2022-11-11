<script lang="ts">
    import { onMount } from 'svelte';
    import init, {upload_excel, test} from '../../../rust/pkg/wasm_game_of_life';

    onMount(async () => {
        // await init()
        await init()
    });

    //Write file parser in rust
    const onFileSelected = async (e)=>{
        // console.log(e.target.files[0])
        // await upload_excel(e.target)
        // await test()
    }

    //Import Grid component and set size of grid
    import Grid from './Grid.svelte'

</script>



<!--<div>-->
<!--    &lt;!&ndash; File input &ndash;&gt;-->
<!--    <input type="file" accept=".csv, .xlsx" on:change={onFileSelected}>-->
<!--</div>-->

<div>
    <!--Display grid-->
    <Grid/>
</div>