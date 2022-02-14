<script>
    import ConnectionDot from "./ConnectionDot.svelte";

    export let state = 0;

    /**
* @param {number} state
*/
    function getStateText(state) {
        switch (state) {
            case 0: return "Disconnected";
            case 1: return "Connecting";
            case 2: return "Connected";
        }
    }

    $: text = getStateText(state); 
</script>



<div class="main">
    <div class="left">
        <ConnectionDot {state} />
    </div>
    <div class="right">
        <span>{text}</span>
    </div>
</div>

<style>
    .main {
        @apply bg-slate-700 grid grid-cols-3 divide-x;
    }

    .right {
        @apply inline-block col-span-2;
    }

    .left {
        @apply inline-block col-span-1;
    }
</style>