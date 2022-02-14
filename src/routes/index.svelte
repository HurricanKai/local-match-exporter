<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { appWindow } from '@tauri-apps/api/window'

    /**
* @param {any} _e
*/
    function fireConnect(_e)
    {
        invoke('connect')
    }

    let arr = []
    appWindow.listen('league-event', (e) => {
        arr = [...arr, e.payload]
        console.log(e)
    })
</script>


<button on:click={fireConnect}>Connect!</button>

{#each arr as el}
    <li>{el.uri} | {JSON.stringify(el.data)}</li>
{/each}