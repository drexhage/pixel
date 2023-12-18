<script lang="ts">
	import { Tool, session, skeleton, tool, ui } from '../../store';
	import OverlayCanvas from './OverlayCanvas.svelte';
	import AlphaPatternCanvas from './AlphaPatternCanvas.svelte';
	import SkeletonCanvas from '../generic/SkeletonCanvas.svelte';
	import ContentCtxCanvas from './ContentCtxCanvas.svelte';
	import ContentCanvas from './ContentCanvas.svelte';

	let height: number;
	let width: number;

	let content: ContentCanvas;
	let overlay: OverlayCanvas;
	let pattern: OverlayCanvas;
	let ctxContent: ContentCtxCanvas;

	export async function redraw() {
		content && content.redraw();
		ctxContent && ctxContent.redraw();
		overlay.redraw();
		pattern.redraw();
	}

	function handleScroll(event: any) {
		let delta = event.deltaY < 0 ? 1.05 : 0.95;
		let [x, y] = [event.offsetX, event.offsetY];
		$ui.zoom_at_position(x, y, delta);
		redraw();
	}

	enum Holding {
		None = 'none',
		Main = 'main',
		Drag = 'drag'
	}

	let holding = Holding.None;
	let mouseX: number;
	let mouseY: number;

	function handleDown(event: MouseEvent) {
		if (event.button === 0) {
			let g = $ui.translate_position(event.offsetX, event.offsetY);
			holding = Holding.Main;
			let x = g[0];
			let y = g[1];
			session.start(g[0], g[1]);
			let old = $ui.translate_position(mouseX, mouseY);
			let prevX = old[0];
			let prevY = old[1];
			session.extend(x, y, prevX, prevY);
		} else {
			holding = Holding.Drag;
		}
	}

	function handleUp(event: MouseEvent) {
		event.preventDefault();
		holding = Holding.None;
		let g = $ui.translate_position(event.offsetX, event.offsetY);
		let x = g[0];
		let y = g[1];
		let old = $ui.translate_position(mouseX, mouseY);
		let prevX = old[0];
		let prevY = old[1];
		session.extend(x, y, prevX, prevY);
		setTimeout(() => {
			session.finish();
			redraw();
		}, 0);
	}

	function handleMove(event: MouseEvent) {
		event.preventDefault();
		if (holding === Holding.Drag) {
			if (mouseX && mouseY) $ui.move_relative(event.offsetX - mouseX, event.offsetY - mouseY);
		} else if (holding === Holding.Main) {
			let g = $ui.translate_position(event.offsetX, event.offsetY);
			let old = $ui.translate_position(mouseX, mouseY);
			let x = g[0];
			let y = g[1];
			let prevX = old[0];
			let prevY = old[1];
			session.extend(x, y, prevX, prevY);
		} else {
			mouseX = event.offsetX;
			mouseY = event.offsetY;
			return;
		}
		redraw();
		mouseX = event.offsetX;
		mouseY = event.offsetY;
	}

	function handleLeave() {
		if (holding === Holding.Main) {
			session.finish();
		}
		holding = Holding.None;
	}

	function handleContextMenu(event: MouseEvent) {
		event.preventDefault();
	}

	let cursor;
	function getCursor(holding) {
		switch (holding) {
			case Holding.Main:
				return $tool === Tool.Move ? 'move' : 'default';
			case Holding.Drag:
				return 'grab';
			default:
				return 'default';
		}
	}
	$: cursor = getCursor(holding);
</script>

{#if !$skeleton}
	<div
		class="wrapper"
		style:cursor
		on:wheel={handleScroll}
		on:mousemove={handleMove}
		on:pointerdown={handleDown}
		on:pointerup={handleUp}
		on:pointerleave={handleLeave}
		on:contextmenu={handleContextMenu}
		bind:clientHeight={height}
		bind:clientWidth={width}
	>
		<AlphaPatternCanvas bind:this={pattern} {height} {width} />
		<!-- <ContentCanvas bind:this={content} {height} {width} /> -->
		<ContentCtxCanvas bind:this={ctxContent} {height} {width} />
		<OverlayCanvas bind:this={overlay} {height} {width} />
	</div>
{:else}
	<SkeletonCanvas />
{/if}

<style>
	.wrapper {
		position: relative;
		overflow: hidden;
		height: 100%;
		width: 100%;
	}
</style>
