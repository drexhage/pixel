<script lang="ts">
	import { ColorScheme, colorScheme, iterColorScheme } from '../../color';
	import Button from './Button.svelte';

	const icons = {
		[ColorScheme.Light]: 'fa-sun',
		[ColorScheme.Dark]: 'fa-moon',
		[ColorScheme.System]: 'fa-system'
	};
</script>

<Button on:click={iterColorScheme}>
	<i class={`fa ${icons[$colorScheme]}`} />
</Button>
