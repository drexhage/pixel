<script lang="ts">
	import { color1, color2, skeleton } from '../../store';
	import SkeletonLoader from '../generic/SkeletonLoader.svelte';

	function flip() {
		let temp = $color1;
		color1.set($color2);
		color2.set(temp);
	}

	function black_white() {
		color1.set('#ffffff');
		color2.set('#000000');
	}
</script>

<div class="color-picker">
	{#if !$skeleton}
		<div class="color-box color2" style="background-color: {$color2};" on:click={flip} on:keydown />
		<label>
			<div class="color-box color1" style="background-color: {$color1};" />
			<input type="color" bind:value={$color1} hidden />
		</label>
	{:else}
		<div class="color-box color2">
			<SkeletonLoader />
		</div>
		<div class="color-box color1">
			<SkeletonLoader />
		</div>
	{/if}
	{#if !$skeleton}
		<div on:click={black_white} on:keydown>
			<div class="default-box default-black" />
			<div class="default-box default-white" />
		</div>
	{/if}
</div>

<style>
	.color-picker {
		display: flex;
		position: relative;
		height: 40px;
		min-width: 40px;
		width: 40px;
		box-sizing: border-box;
	}

	.color-box {
		position: absolute;
		display: inline-block;
		height: 30px;
		width: 30px;
		border-radius: 5px;
		background-color: #ff0000;
		box-sizing: border-box;
		border: solid var(--color-stronger) 1px;
	}

	.color2 {
		top: 10px;
		left: 10px;
	}

	.color1 {
		top: 0;
		left: 0;
		cursor: pointer;
	}

	.default-box {
		position: absolute;
		display: inline-block;
		width: 6px;
		height: 6px;
		box-sizing: border-box;
		border: solid var(--color-stronger) 1px;
	}

	.default-white {
		background-color: white;
		left: 0;
		bottom: 2px;
	}

	.default-black {
		background-color: black;
		left: 2px;
		bottom: 0;
	}
</style>
