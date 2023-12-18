<script lang="ts">
	import { skeleton } from '../../store';
	import SkeletonLoader from './SkeletonLoader.svelte';

	export let value;
	export let name: string;
	export let min: number;
	export let max: number;
	export let step: number = 1;
	export let delta: number = 0.07;
	export let fmt: (x: any) => String = (x) => x;

	function onWheel(e) {
		if (e.deltaY > 0) {
			value = Math.max(value - max * delta, min);
		} else if (e.deltaY < 0) {
			value = Math.min(value + max * delta, max);
		}
		// without rounding there will be 7.999.. etc happen from the above
		value = Math.round(1_000_000 * value) / 1_000_000;
	}
</script>

{#if $skeleton}
	<div class="box">
		<SkeletonLoader />
	</div>
{:else}
	<div class="box" on:wheel={onWheel}>
		<div class="selected" style={'width: ' + (100 * value) / max + '%'} />
		<div class="overlay">
			<span>
				{name}
			</span>
			<span>
				{fmt(value)}
			</span>
		</div>
		<input type="range" {min} {max} {step} bind:value class="slider" name="slider" id="slider" />
	</div>
{/if}

<style>
	.slider {
		position: absolute;
		-webkit-appearance: none;
		appearance: none;
		width: 100%;
		height: 100%;
		background: transparent;
		outline: none;
		margin: 0;
		transition: all 0.2s;
	}
	.slider:hover {
		background: rgba(#f2f2f2, 0.1);
		cursor: ew-resize;
	}
	.slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 6px;
		height: 100%;
		background: var(--color-lightest);
		cursor: ew-resize;
	}
	.slider::-moz-range-thumb {
		-moz-appearance: none;
		width: 6px;
		height: 100%;
		background: var(--color-lightest);
		cursor: ew-resize;
	}

	.box {
		display: flex;
		overflow: hidden;
		align-items: center;
		text-align: center;
		position: relative;
		background-color: var(--color-lightest);
		width: 100%;
		border: 1px solid var(--color-stronger);
		cursor: col-resize;
		box-sizing: border-box;
		height: 27px;
		border-radius: 5px;
		&:hover > .selected {
			background-color: var(--color-light);
		}
	}
	.selected {
		background-color: var(--color-lighter);
		position: absolute;
		height: 100%;
		top: 0;
		left: 0;
		border-right: 1px solid var(--color-stronger);
		border-radius: 5px;
		box-sizing: border-box;
	}
	.overlay {
		user-select: none;
		display: flex;
		top: 0;
		left: 0;
		width: 100%;
		padding: 0 10px;
		box-sizing: border-box;
		justify-content: space-between;
		align-items: center;
		position: absolute;
		height: 25px;
	}
</style>
