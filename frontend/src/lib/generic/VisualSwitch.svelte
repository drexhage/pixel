<script lang="ts">
	import { skeleton } from '../../store';
	import SkeletonLoader from './SkeletonLoader.svelte';
	import { shortcut } from '../../shortcut';

	export let value;
	export let options: { value: any; title: string; icon: string; shortcut: any }[];
	export let onChange;
</script>

{#if !$skeleton}
	<div class="box">
		{#each options as option}
			<button
				class={value == option.value ? 'selected-btn' : 'control-btn'}
				title={option.title}
				use:shortcut={option.shortcut}
				on:click={() => onChange(option.value)}><i class={'fa ' + option.icon} /></button
			>
		{/each}
	</div>
{:else}
	<div class="box skeleton">
		<SkeletonLoader />
	</div>
{/if}

<style>
	.box.skeleton {
		padding: 0px;
	}
	.box {
		display: flex;
		gap: 5px;
		width: 100%;
		height: 40px;
		border-radius: 10px;
		overflow: hidden;
		padding: 5px;
		box-sizing: border-box;
		background-color: var(--color-lighter);
		border: 1px solid var(--color-stronger);
	}

	.selected-btn {
		color: var(--color-strongest);
		background: var(--color-lightest);
		border: 1px solid var(--color-stronger);

		border-radius: 7px;
		width: 100%;
		box-sizing: border-box;
		outline: none;
		&:hover {
			cursor: pointer;
			background: var(--color-light);
		}
	}

	.control-btn {
		border-radius: 7px;
		height: 100%;
		width: 100%;
		color: var(--color-strong);
		outline: none;
		border: none;
		box-sizing: border-box;
		background: transparent;
		&:hover {
			border: 1px solid var(--color-strongest);
			cursor: pointer;
			background: var(--color-light);
		}
	}
</style>
