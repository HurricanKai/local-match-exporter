<script>
    import { emit, listen } from '@tauri-apps/api/event'
    import { invoke } from '@tauri-apps/api/tauri'

    /**
* @param {any} _e
*/
    function fireConnect(_e)
    {
        invoke('connect')
    }
</script>


<div>
    <button on:click={fireConnect}>Connect!</button>
</div>