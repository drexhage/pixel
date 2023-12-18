<script lang="ts">
	import { skeleton } from '../../store';
	import SkeletonLoader from './SkeletonLoader.svelte';

	export let value;
	export let title;
	export let options: { key: string; value }[];
</script>

{#if !$skeleton}
	<div class="box">
		<select bind:value on:change>
			{#each options as { key, value }, _}
				<option>{value}</option>
			{/each}
		</select>
		<div class="title">
			<span>{title}</span>
		</div>
	</div>
{:else}
	<div class="box">
		<SkeletonLoader />
	</div>
{/if}

<style>
	.box {
		display: inline-block;
		width: 100%;
		height: 27px;
		position: relative;
		border: 1px solid var(--color-stronger);
		border-radius: 5px;
		overflow: hidden;
		box-sizing: border-box;
	}

	.title {
		display: flex;
		align-items: center;
		position: absolute;
		left: 10px;
		top: 0;
		height: 100%;
	}

	select {
		display: inline-block;
		width: 100%;
		background-color: var(--color-lightest);
		font: inherit;
		padding: 0 5px;
		height: 27px;
		outline: none;
		border: none;
		cursor: pointer;
		box-sizing: border-box;
		color: var(--color-strongest);
		text-align: end;
	}

	option {
		cursor: pointer;
	}
</style>
