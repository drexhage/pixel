<script lang="ts">
	import { skeleton } from '../../store';
	import SkeletonLoader from './SkeletonLoader.svelte';

	export let disabled = false;
	export let title = undefined;
	export let style = undefined;
</script>

{#if $skeleton}
	<div class="btn skeleton">
		<SkeletonLoader />
	</div>
{:else}
	<button type="button" class="btn" on:click {title} {disabled} {style}><slot /></button>
{/if}

<style>
	.btn.skeleton {
		border-color: var(--color-strong);
	}
	.btn {
		overflow: hidden;
		position: relative;
		display: inline-block;
		height: 27px;
		width: 27px;
		outline: none;
		border: none;
		background: var(--color-lightest);
		color: var(--color-stronger);
		border: 1px solid var(--color-stronger);
		border-radius: 5px;
		box-sizing: border-box;
		&:hover:not(:disabled) {
			cursor: pointer;
			background: var(--color-light);
		}
		&:disabled {
			color: var(--color-strong);
			background: var(--color-lighter);
		}
		&:disabled:hover {
			cursor: not-allowed;
		}
	}
</style>
