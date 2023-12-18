<script lang="ts">
	import { onMount } from 'svelte';
	import { session, ui } from '../../store';

	export let width: number;
	export let height: number;

	let canvas: HTMLCanvasElement;
	let offscreen: HTMLCanvasElement;
	let ctx, offscreenCtx;

	export function redraw() {
		if (!ctx) return;
		let clamped;
		try {
			clamped = session.content_as_bytes();
		} catch (e) {
			console.log(e);
			return;
		}
		let imageData = new ImageData(clamped, $ui.img_size.width, $ui.img_size.height);

		offscreenCtx = offscreen.getContext('2d');
		offscreenCtx.imageSmoothingEnabled = false;
		offscreenCtx.clearRect(0, 0, offscreen.width, offscreen.height);
		offscreenCtx.putImageData(imageData, 0, 0);

		ctx.imageSmoothingEnabled = false;
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		let scale = $ui.scale;
		ctx.drawImage(
			offscreen,
			$ui.position.x,
			$ui.position.y,
			$ui.img_size.width * scale,
			$ui.img_size.height * scale
		);
	}

	onMount(() => {
		ctx = canvas.getContext('2d');
		offscreen = document.createElement('canvas');
		redraw();
		sizeChanged();
		session.add_redraw_callback(redraw);
	});

	function sizeChanged() {
		$ui.set_canvas_size(width, height);
		$ui.center();
		if (offscreen) {
			offscreen.width = $ui.img_size.width;
			offscreen.height = $ui.img_size.height;
			offscreenCtx = offscreen.getContext('2d');
		}
		redraw();
	}
	$: width && sizeChanged();
	$: height && sizeChanged();
</script>

<canvas id="canvas-content" bind:this={canvas} {width} {height} />

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
