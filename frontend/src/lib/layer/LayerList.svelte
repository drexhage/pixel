<script lang="ts">
	import { layers } from '../../store';
	import DragDropIndicator from './DragDropIndicator.svelte';
	import LayerTreeEntry from './LayerTreeEntry.svelte';
	import LevelPadding from './LevelPadding.svelte';

	export let children: any[];
	export let move_idx: number = 0;
	export let level: number = 0;
</script>

<div>
	<LevelPadding {level} />
	<DragDropIndicator {move_idx} {level} />
	<ul class="layers">
		{#each { length: children.length } as _, i}
			{@const idx = children[children.length - 1 - i]}
			{@const child = $layers.nodes[idx]}
			<LayerTreeEntry node={child} {level} />
		{/each}
	</ul>
</div>

<style>
	ul.layers {
		display: flex;
		align-items: flex-start;
		flex-direction: column;
		list-style-type: none;
		padding: 0;
		margin: 0;
	}
</style>
