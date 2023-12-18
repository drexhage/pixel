<script lang="ts">
	import { ColorScheme, colorScheme, systemLightScheme } from '../color';

	let cssFile = 'light';
	$: cssFile =
		$colorScheme === ColorScheme.System ? ($systemLightScheme ? 'light' : 'dark') : $colorScheme;
</script>

<svelte:head>
	<meta name="color-scheme" content={$colorScheme} />
	<link rel="stylesheet" href={`/theme/${cssFile}.css`} />
</svelte:head>

<slot />
