<script lang="ts">
	import { BlendMode, focused, layers, radius, session, skeleton } from '../../store';
	import LayerControls from './LayerControls.svelte';
	import Selection from '../generic/Selection.svelte';
	import Slider from '../generic/Slider.svelte';
	import LayerList from './LayerList.svelte';
	import type { Layer, Node } from '../../types';
	import Separator from '../generic/Separator.svelte';
	import SkeletonLoader from '../generic/SkeletonLoader.svelte';

	let selected: number | null = null;
	$: selected = $focused.length === 1 ? $focused[0] : null;

	let layer: Node<Layer> | null = null;
	$: selected && setLayerAndAlpha(selected);

	let alpha = 0;
	let blendMode: string = BlendMode.Alpha;

	function setLayerAndAlpha(selected) {
		layer = selected ? $layers.nodes[selected] : null;
		alpha = layer && layer.value.attr.alpha;
		blendMode = layer && layer.value.attr.mode;
	}

	function setAlpha(a) {
		if (!layer) return;
		if (alpha == layer.value.attr.alpha) return;
		session.perform_step({
			type: 'layer/attr',
			id: layer.id,
			alpha: a
		});
	}
	$: setAlpha(alpha);

	function setBlendMode(blendMode) {
		if (!layer) return;
		if (blendMode == layer.value.attr.mode) return;
		session.perform_step({
			type: 'layer/attr',
			id: layer.id,
			mode: blendMode
		});
	}
	$: setBlendMode(blendMode);
</script>

{#key $layers}
	<div id="layers-box">
		{#if selected || $skeleton}
			<div>
				<div id="layers-attributes">
					<Slider
						min={0}
						max={1}
						step={0.01}
						bind:value={alpha}
						name="alpha"
						fmt={(x) => `${~~(x * 100)}%`}
					/>
					<Selection
						title="mode"
						bind:value={blendMode}
						options={[
							{ key: null, value: BlendMode.Alpha },
							{ key: null, value: BlendMode.Remove },
							{ key: null, value: BlendMode.Lighten },
							{ key: null, value: BlendMode.Darken },
							{ key: null, value: BlendMode.Screen }
						]}
					/>
				</div>
				<Separator />
			</div>
		{:else}
			<div />
		{/if}
		{#if $skeleton}
			<SkeletonLoader />
		{:else}
			<div id="layer-list-box">
				<div id="layers-list">
					<LayerList children={$layers.nodes[$layers.root].children} />
				</div>
			</div>
		{/if}
		<Separator />
		<div>
			<LayerControls />
		</div>
	</div>
{/key}

<style>
	#layers-list {
		width: 100%;
		height: 100%;
		overflow-y: auto;
		flex-grow: 0;
		flex-shrink: 0;
		position: absolute;
		/* width */
		&::-webkit-scrollbar {
			width: 10px;
		}

		/* Track */
		&::-webkit-scrollbar-track {
			background: var(--color-lightest);
		}

		/* Handle */
		&::-webkit-scrollbar-thumb {
			background: var(--color-lighter);
		}

		/* Handle on hover */
		&::-webkit-scrollbar-thumb:hover {
			background: var(--color-light);
		}
	}

	#layers-box {
		display: grid;
		grid-template-rows: auto 1fr auto auto;
		height: 100%;
		overflow: hidden;
		box-sizing: border-box;
	}

	#layer-list-box {
		position: relative;
		display: grid;
		grid-template-rows: auto 1fr auto auto;
		height: 100%;
		overflow: hidden;
		box-sizing: border-box;
	}

	#layers-attributes {
		display: grid;
		gap: 5px;
		padding: 5px;
		grid-template-columns: 1fr 1.2fr;
	}
</style>
