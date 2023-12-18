<script lang="ts">
	import { draggingLayers, session } from '../../store';
	import LevelPadding from './LevelPadding.svelte';

	export let move_idx: number;
	export let level: number;

	let droppable = false;
	$: droppable = $draggingLayers ? droppable : false;

	function allowDrop(ev) {
		ev.preventDefault();
		droppable = true;
	}

	function disallowDrop(_ev) {
		droppable = false;
	}

	function onDrop(_ev) {
		let id = $draggingLayers;
		if (id == null) return;
		session.perform_step({
			type: 'layer/move',
			id,
			move_idx
		});
		draggingLayers.set(null);
	}
</script>

<div class="box">
	<LevelPadding {level} />
	{#if $draggingLayers}
		{#if !droppable}
			<div class="small-indicator" />
		{/if}
		<div
			class="indicator"
			style={`background: ${droppable ? 'darkgray' : 'transparent'};`}
			on:dragover={allowDrop}
			on:dragleave={disallowDrop}
			on:drop={onDrop}
		/>
	{/if}
</div>

<style>
	.small-indicator {
		position: absolute;
		top: -2px;
		left: 20px;
		right: 20px;
		height: 4px;
		background-color: darkgray;
	}
	.indicator {
		position: absolute;
		top: -10px;
		left: 0;
		height: 20px;
		width: 100%;
	}
	.box {
		position: relative;
	}
</style>
