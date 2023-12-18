<script lang="ts">
	import { onMount } from 'svelte';
	import { session, ui } from '../../store';
	import { colorScheme, systemLightScheme } from '../../color';

	export let width: number;
	export let height: number;

	let canvas: HTMLCanvasElement;
	let bg = '#000';

	export function redraw() {
		if (!canvas) return;
		const ctx = canvas.getContext('2d')!;
		if (!ctx) return;
		ctx.clearRect(0, 0, width, height);
		mask(ctx);
		borderContent(ctx);
	}

	function mask(ctx) {
		ctx.fillStyle = bg;
		ctx.fillRect(0, 0, width, height);
		ctx.clearRect(
			$ui.position.x,
			$ui.position.y,
			$ui.img_size.width * $ui.scale,
			$ui.img_size.height * $ui.scale
		);
	}

	function borderContent(ctx) {
		ctx.fillStyle = '#FF0000';
		ctx.beginPath();
		ctx.moveTo($ui.position.x, $ui.position.y);
		ctx.lineTo($ui.position.x + $ui.img_size.width * $ui.scale, $ui.position.y);
		ctx.lineTo(
			$ui.position.x + $ui.img_size.width * $ui.scale,
			$ui.position.y + $ui.img_size.height * $ui.scale
		);
		ctx.lineTo($ui.position.x, $ui.position.y + $ui.img_size.height * $ui.scale);
		ctx.lineTo($ui.position.x, $ui.position.y);
		ctx.stroke();
	}

	function sizeChanged() {
		redraw();
	}
	$: width && sizeChanged();
	$: height && sizeChanged();

	function updateBackground(...args) {
		setTimeout(() => {
			bg = getComputedStyle(document.documentElement).getPropertyValue('--color-lighter');
			redraw();
		}, 100);
	}

	onMount(() => {
		redraw();
		updateBackground();
		session.add_redraw_callback(redraw);
	});

	$: updateBackground($colorScheme, $systemLightScheme);
</script>

<canvas id="canvas-overlay" bind:this={canvas} {width} {height} />

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
