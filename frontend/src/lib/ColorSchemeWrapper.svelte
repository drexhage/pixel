<script>
	import { ColorScheme, colorScheme, systemLightScheme } from '../color';
	import { base } from '$app/paths';

	let cssFile = 'light';
	$: cssFile =
		$colorScheme === ColorScheme.System ? ($systemLightScheme ? 'light' : 'dark') : $colorScheme;
</script>

<svelte:head>
	<meta name="color-scheme" content={$colorScheme} />
	<link rel="stylesheet" href={`${base}/theme/${cssFile}.css`} />
</svelte:head>

<slot />
