<script lang="ts">
    import { Rive, Layout, Fit } from "@rive-app/webgl";
	import { onMount } from 'svelte';

    export let state : Number = 0;
    let inputState;
    let canvas;

    onMount(() => {
        let dot = new Rive({
            src: '/connection_dot.riv',
            canvas: canvas,
            autoplay: true,
            stateMachines: 'State Machine 1',
            layout: new Layout({
                fit: Fit.Contain
            }),
            onLoad: (_) => {
                console.log(dot);

                let inputs = dot.stateMachineInputs('State Machine 1');
                console.log(inputs);
                inputState = inputs.find(i => i.name === 'State');
            }
        });
    });

    $: if (inputState != undefined) { inputState.value = state; }
</script>

<div>
    <canvas bind:this={canvas}></canvas>
</div>

<style>
    canvas {
        width:100%;
        height:100%;
    }
</style>