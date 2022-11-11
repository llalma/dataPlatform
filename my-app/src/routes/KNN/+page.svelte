<script>
    import papa from 'papaparse';

    let data, fileName;
    let headers = [];

    let inputColumns = [];
    let resultColumns = [];
    let x, y;

    // Upload File function
    const onFileSelected = async (e)=>{
        papa.parse(
            e.target.files[0],
            {header: true,
            complete: function (res) {
                headers = res.meta['fields']
                data = res.data
                fileName = e.target.files[0].name
                inputColumns = headers.slice(0, headers.length-1)
                resultColumns =  headers.slice(-1)
            }
            }
        );
    }

    // Select new result column
    let selected;

	function handleSubmit() {
		alert(`${selected}`);
	}

    import MultiSelect from 'svelte-multiselect'

</script>

<!-- Button to select file to upload -->
<input type="file" accept=".csv, .xlsx" on:change={onFileSelected}>
{#if data}

    <!-- Access Data -->
    <!-- <p>{data[0].A}</p> -->

    <table>

        <tr>
            <!-- Display file which was uploaded -->
            <td>Uploaded File:</td>
            <td><b>{fileName}</b></td>
        </tr>

        <tr>
            <!-- Display all columns besides last as input data -->
            <td>Input Columns:</td>
            <td>
                <MultiSelect bind:selected={inputColumns} options={headers}/>
            </td>
        </tr>

        <tr>
        <!-- Display last column as result col by deafult -->
            <td>Result Column: </td>
            <td><MultiSelect bind:selected={resultColumns} options={headers}/></td>
        </tr>


    </table>

{:else}
    <p>No File uploaded</p>
{/if}

