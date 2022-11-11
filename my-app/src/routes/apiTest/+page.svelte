<script>

    let remoteProcessing = false;

    let localURL = 'http://127.0.0.1:8000'
    let remoteURL = 'https://03s0cqi1b7.execute-api.us-east-1.amazonaws.com/dev'

    let baseURL = 'http://127.0.0.1:8000'

    const toggleRemoteProcessing = () => {
        remoteProcessing = !remoteProcessing;
        if (remoteProcessing){
            baseURL = remoteURL
        }else{
            baseURL = localURL
        }
    }

	async function KNNtrain() {
        const testBody = {
            x:1
        }

        await fetch(
            `${baseURL}/KNN/train`, {method: 'POST', body:JSON.stringify(testBody)})
            .then(res => res.json())
            .then(data => {console.log(data)})
	}

    async function KNNpredict() {
        const testBody = {
            x:2
        }

        await fetch(
            `${baseURL}/KNN/predict`, {method: 'POST', body:JSON.stringify(testBody)})
            .then(res => res.json())
            .then(data => {console.log(data)})
	}

    async function RFtrain() {
        const testBody = {
            x:3
        }

        await fetch(
            `${baseURL}/RF/train`, {method: 'POST', body:JSON.stringify(testBody)})
            .then(res => res.json())
            .then(data => {console.log(data)})
	}

    async function RFpredict() {
        const testBody = {
            x:4
        }

        await fetch(
            `https://hkc4toziw3.execute-api.us-east-1.amazonaws.com/call/`, {method: 'POST', body:JSON.stringify(testBody)})
            .then(res => res.json())
            .then(data => {console.log(data)})
	}

</script>

<!-- Test -->

<!-- Toggle for source -->
<table>
    <td>Remote Processing</td>
    <td><input type=checkbox on:click={toggleRemoteProcessing}></td>
</table>

<!-- Buttons for requesting -->
<table>
    <td><button on:click={KNNtrain}>KNNtrain</button></td>
    <td><button on:click={KNNpredict}>KNNpredict</button></td>
    <td><button on:click={RFtrain}>RFtrain</button></td>
    <td><button on:click={RFpredict}>RFpredict</button></td>
</table>
<!-- /Test -->