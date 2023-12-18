<script lang="ts">
	import InteractiveCanvas from '../canvas/InteractiveCanvas.svelte';
	import Button from '../generic/Button.svelte';
	import Dialog from '../generic/Dialog.svelte';
	import LayerComponent from '../layer/LayerComponent.svelte';
	import ToolControls from '../tool/ToolControls.svelte';
	import ToolSelection from '../tool/ToolSelection.svelte';

	let showLayer = false;
	let showBottomDrawer = false;
</script>

<div class="layout">
	<InteractiveCanvas />

	<Dialog title={'Layer'} bind:show={showLayer}>
		<LayerComponent />
	</Dialog>

	<div id="control" style:bottom={showBottomDrawer ? '0px' : '-430px'}>
		<div class="thumb-box" on:click={() => (showBottomDrawer = !showBottomDrawer)} on:keydown>
			<div class="thumb" />
		</div>
		<div class="thumb-control">
			<ToolSelection />
			<Button
				on:click={() => {
					showLayer = true;
				}}><i class="fa fa-folder" /></Button
			>
		</div>
		<ToolControls />
	</div>
</div>

<style>
	.thumb {
		height: 5px;
		width: 50px;
		background-color: lightgray;
		border-radius: 10px;
	}

	.thumb-control {
		display: grid;
		grid-template-columns: 1fr auto;
		align-items: center;
		gap: 5px;
	}

	.thumb-box {
		display: flex;
		justify-content: center;
		align-items: center;
		align-content: center;
		width: 100%;
	}

	.layout {
		position: relative;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
	}

	#control {
		position: absolute;
		padding: 5px;
		display: flex;
		flex-direction: column;
		gap: 5px;
		left: 0;
		width: 100%;
		height: 500px;
		background-color: var(--color-lightest);
		box-sizing: border-box;
		border: 1px solid gray;
		border-top-left-radius: 10px;
		border-top-right-radius: 10px;
	}
</style>
