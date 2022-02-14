<script>
    import { emit, listen } from '@tauri-apps/api/event'
    import { invoke } from '@tauri-apps/api/tauri'
    import ConnectionIndicator from '$lib/components/ConnectionIndicator.svelte';

    /**
* @param {any} _e
*/
    function fireConnect(_e)
    {
        invoke('connect')
    }
</script>


<div class="w-60">
    <button on:click={fireConnect}>Connect!</button>
    <ConnectionIndicator state=1/>
</div>