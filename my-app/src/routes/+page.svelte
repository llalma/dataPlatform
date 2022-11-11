<script>

const redirects = {Home: "/", UploadandPlot: "/UploadandPlot"}

</script>

<h1>Welcome to SvelteKit</h1>

<!-- Create Nav bar -->
<nav>
    {#each Object.entries(redirects) as [key, value]}
        <a href={value}>{key}</a><br>
    {/each}
</nav>