<script>
    import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
    import ConnectionIndicator from './ConnectionIndicator.svelte';
</script>

<Meta title="Connection Indicator" component={ConnectionIndicator}/>

<Template let:args>
  <ConnectionIndicator {...args}/>
</Template>

<Story name="Disconnected" args={{state: 0}}/>

<Story name="Connecting" args={{state: 1}}/>

<Story name="Connected" args={{state: 2}}/>