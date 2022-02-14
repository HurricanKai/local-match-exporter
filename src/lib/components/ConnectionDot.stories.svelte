<script>
    import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
    import ConnectionDot from './ConnectionDot.svelte';
</script>

<Meta title="Connection Dot" component={ConnectionDot}/>

<Template let:args>
  <ConnectionDot {...args}/>
</Template>

<Story name="Disconnected" args={{state: 0}}/>

<Story name="Connecting" args={{state: 1}}/>

<Story name="Connected" args={{state: 2}}/>