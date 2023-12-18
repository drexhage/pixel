<script lang="ts">
	import { focused, session } from '../../store';
	import Button from '../generic/Button.svelte';
	import ButtonGroup from '../generic/ButtonGroup.svelte';

	function newLayer() {
		if ($focused.length > 1) return;
		let move_idx = $focused.length === 1 ? $focused[0] : 0;
		session.perform_step({
			type: 'layer/create/empty',
			move_idx,
			position: null
		});
	}

	function newGroup() {
		if ($focused.length > 1) return;
		let move_idx = $focused.length === 1 ? -$focused[0] : 0;
		session.perform_step({
			type: 'layer/create/group',
			move_idx
		});
	}

	function remove() {
		session.perform_step({
			type: 'layer/remove',
			ids: $focused
		});
		focused.set([]);
	}

	function createLayerFile(e) {
		let image = e.target.files[0];
		let reader = new FileReader();
		reader.readAsDataURL(image);
		reader.onload = (e) => {
			let data = e.target.result;
			data = data.slice('data:image/png;base64,'.length);
			session.perform_step({
				type: 'layer/create/from_data',
				parent: 0,
				img: {
					src: 'encode/png',
					data: data
				}
			});
		};
	}

	function moveLayerUp() {
		if ($focused.length !== 1) return;
		let id = $focused[0];
		session.move_layer_up(id);
	}

	function moveLayerDown() {
		if ($focused.length !== 1) return;
		let id = $focused[0];
		session.move_layer_down(id);
	}

	function mergeDown() {
		if ($focused.length !== 1) return;
		let id = $focused[0];
		session.perform_step({
			type: 'layer/merge_down',
			id
		});
	}

	function duplicateLayer() {
		if ($focused.length == 0) return;
		session.perform_step({
			type: 'layer/duplicate',
			id: $focused[0]
		});
	}

	let noneFocused = true;
	$: noneFocused = $focused.length === 0;
</script>

<div id="layer-controls">
	<ButtonGroup>
		<Button on:click={newGroup}><i class="fa fa-folder" /></Button>
		<Button on:click={newLayer}><i class="fa fa-file" /></Button>
		<label for="myfile">
			<input
				type="file"
				id="myfile"
				style="display: none;"
				name="myfile"
				on:change={createLayerFile}
			/>
			<Button style={'pointer-events: none'} on:click={() => {}}><i class="fa fa-image" /></Button>
		</label>
	</ButtonGroup>
	<ButtonGroup>
		<Button on:click={moveLayerUp} disabled={noneFocused}><i class="fa fa-arrow-up" /></Button>
		<Button on:click={moveLayerDown} disabled={noneFocused}><i class="fa fa-arrow-down" /></Button>
	</ButtonGroup>
	<ButtonGroup>
		<Button on:click={remove} disabled={noneFocused}><i class="fa fa-trash" /></Button>
		<Button on:click={mergeDown} disabled={noneFocused}><i class="fa fa-random" /></Button>
		<Button on:click={duplicateLayer} disabled={noneFocused}><i class="fa fa-copy" /></Button>
	</ButtonGroup>
</div>

<style>
	#layer-controls {
		padding: 5px;
		display: flex;
		justify-content: space-between;
	}
</style>
