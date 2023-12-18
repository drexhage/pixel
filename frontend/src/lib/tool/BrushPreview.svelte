<script lang="ts">
	import { Tool, alpha, color1, hardness, skeleton, tool } from '../../store';
	import AlphaPatternCanvas from '../canvas/AlphaPatternCanvas.svelte';
	import SkeletonLoader from '../generic/SkeletonLoader.svelte';

	export let width;
	export let height;

	let brushPreview: HTMLCanvasElement;
	let alphaBackground: AlphaPatternCanvas;

	function updateBrush(
		canvas: HTMLCanvasElement,
		alpha: number,
		hardness: number,
		color: string,
		invert: boolean
	) {
		if (!canvas) return;
		let ctx = canvas.getContext('2d');
		let outerRadius = Math.min(height, width) / 2;
		let innerRadius = outerRadius * hardness;
		const imgData: ImageData = ctx.createImageData(width, height);
		let data = imgData.data;
		let [r, g, b] = hexToRgb(color);
		for (let y = 0; y < height; y++) {
			for (let x = 0; x < width; x++) {
				let i = 4 * (y * width + x);
				data[i] = r;
				data[i + 1] = g;
				data[i + 2] = b;
				data[i + 3] = 0;
				let distance = Math.sqrt((x - width / 2) ** 2 + (y - height / 2) ** 2);
				if (invert) {
					data[i + 3] = 255 - alpha;
					if (distance <= outerRadius && distance > innerRadius) {
						let extraAlpha = (distance - innerRadius) / (outerRadius - innerRadius);
						data[i + 3] = Math.min(~~(255 * extraAlpha) + data[i + 3], 255);
					} else if (distance > outerRadius) {
						data[i + 3] = 255;
					}
				} else {
					if (distance <= innerRadius) {
						data[i + 3] = alpha;
					} else if (distance <= outerRadius) {
						let extraAlpha = 1 - (distance - innerRadius) / (outerRadius - innerRadius);
						data[i + 3] = ~~(alpha * extraAlpha);
					}
				}
			}
		}
		ctx.putImageData(imgData, 0, 0);
	}

	function hexToRgb(hex: string) {
		let bigint = parseInt(hex.substring(1), 16);
		let r = (bigint >> 16) & 255;
		let g = (bigint >> 8) & 255;
		let b = bigint & 255;

		return [r, g, b];
	}

	$: updateBrush(brushPreview, $alpha, $hardness, $color1, $tool === Tool.Eraser);
</script>

{#if !$skeleton}
	<div class="box" style:width={`${width}px`} style:height={`${height}px`}>
		<AlphaPatternCanvas {height} {width} bind:this={alphaBackground} />
		<canvas id="brush-preview" {height} {width} bind:this={brushPreview} />
	</div>
{:else}
	<div class="box" style:width={`${width}px`} style:height={`${height}px`}>
		<SkeletonLoader />
	</div>
{/if}

<style>
	.box {
		position: relative;
		border: 1px solid var(--color-stronger);
		border-radius: 5px;
		overflow: hidden;
	}
	#brush-preview {
		position: absolute;
		top: 0;
		left: 0;
	}
</style>
