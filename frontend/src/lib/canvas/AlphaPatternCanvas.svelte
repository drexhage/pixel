<script lang="ts">
	import { onMount } from 'svelte';

	export let width: number;
	export let height: number;

	let canvas: HTMLCanvasElement;
	let pattern: HTMLCanvasElement;

	function init() {
		pattern = document.createElement('canvas');
		let a = 32;

		pattern.width = a;
		pattern.height = a;
		let ctx = pattern.getContext('2d');

		ctx.fillStyle = '#eee';
		ctx.fillRect(0, 0, a, a);
		ctx.fillStyle = '#ccc';
		ctx.fillRect(0, 0, a / 2, a / 2);
		ctx.fillRect(a / 2, a / 2, a / 2, a / 2);
	}

	export function redraw() {
		if (!canvas) return;
		let ctx = canvas.getContext('2d');
		if (!ctx) return;
		ctx.beginPath();
		let pat = ctx.createPattern(pattern, 'repeat');
		ctx.fillStyle = pat;
		ctx.fillRect(0, 0, width, height);
	}

	onMount(() => {
		init();
		redraw();
	});
</script>

<canvas id="canvas-alpha-pattern" bind:this={canvas} {width} {height} />

<style>
	canvas {
		position: absolute;
		top: 0;
		left: 0;
		display: block;
		width: 100%;
		height: 100%;
	}
</style>
