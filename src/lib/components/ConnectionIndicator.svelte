<script lang="ts">
    import ConnectionDot from "./ConnectionDot.svelte";

    export let state : Number = 0;

    function getStateText(state : Number) : String {
        if (state == 0) return "Disconnected";
        if (state == 1) return "Connecting";
        if (state == 2) return "Connected";
        console.log(state);
        return "ERROR_INVALID_STATE";
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

    .right span {
        @apply text-center
    }

    .left {
        @apply inline-block col-span-1;
    }
</style>