<script lang="ts">
	import { blender, layers, redoable, session, ui, undoable } from '../../store';
	import InteractiveCanvas from '../canvas/InteractiveCanvas.svelte';
	import Button from '../generic/Button.svelte';
	import ButtonGroup from '../generic/ButtonGroup.svelte';
	import Card from '../generic/Card.svelte';
	import History from '../History.svelte';
	import LayerComponent from '../layer/LayerComponent.svelte';
	import ToolControls from '../tool/ToolControls.svelte';
	import ToolSelection from '../tool/ToolSelection.svelte';
	import Separator from '../generic/Separator.svelte';
	import Selection from '../generic/Selection.svelte';
	import Dialog from '../generic/Dialog.svelte';

	function base64ToArrayBuffer(base64) {
		let binaryString = window.atob(base64);
		let binaryLen = binaryString.length;
		let bytes = new Uint8Array(binaryLen);
		for (let i = 0; i < binaryLen; i++) {
			let ascii = binaryString.charCodeAt(i);
			bytes[i] = ascii;
		}
		return bytes;
	}

	function imageBytesToBase64(image_data, width, height) {
		let canvas = document.createElement('canvas');
		canvas.width = width;
		canvas.height = height;
		let ctx = canvas.getContext('2d')!;
		ctx.putImageData(image_data, 0, 0);
		let base64 = canvas.toDataURL('image/png').slice('data:image/png;base64,'.length);
		return base64;
	}

	function saveFile(fileName, urlFile) {
		let a = document.createElement('a');
		document.body.appendChild(a);
		a.href = urlFile;
		a.download = fileName;
		a.click();
		a.remove();
	}

	const opts = {
		types: [
			{
				description: 'Image file (png)',
				accept: { 'image/png': ['.png'] }
			}
		],
		suggestedName: 'result.png'
	};

	async function download() {
		let width = $ui.img_size.width;
		let height = $ui.img_size.height;
		let pointer = $layers.nodes[$layers.root].value.pointer;
		let preview_content = session.get_content_of(pointer, width, height);
		let image_data = new ImageData(preview_content, width, height);
		let base64 = imageBytesToBase64(image_data, width, height);
		let sampleArr = base64ToArrayBuffer(base64);
		let blob = new Blob([sampleArr], { type: 'image/png' });
		let url = window.URL.createObjectURL(blob);

		let showSaveFilePicker = window['showSaveFilePicker'];
		if (showSaveFilePicker) {
			// only supported by chrome :(
			try {
				const handle = await showSaveFilePicker(opts);
				const writable = await handle.createWritable();
				await writable.write(blob);
				writable.close();
			} catch (e) {
				// who cares
			}
		} else {
			saveFile('result.png', url);
		}
		window.URL.revokeObjectURL(url);
	}

	let show = false;
</script>

<div class="layout">
	<Dialog bind:show>
		<History />
	</Dialog>

	<!-- SIDEBAR -->
	<div id="sidebar">
		<div />
		<div class="top-tooling">
			<Selection
				value={$blender}
				on:change={(e) => session.switch_blender(e.target['value'])}
				title="blender"
				options={[
					{ key: 'Software', value: 'Software' },
					{ key: 'WebGL', value: 'WebGL' }
				]}
			/>
			<Button on:click={download} title="download"><i class="fa fa-download" /></Button>
			<ButtonGroup>
				<Button on:click={session.undo} disabled={!$undoable} title="undo"
					><i class="fa fa-arrow-left" /></Button
				>
				<Button on:click={session.redo} disabled={!$redoable} title="redo"
					><i class="fa fa-arrow-right" /></Button
				>
			</ButtonGroup>
		</div>
		<Separator />
		<ToolSelection />
		<Card>
			<ToolControls />
		</Card>

		<Card>
			<LayerComponent />
		</Card>
		<div />
	</div>

	<InteractiveCanvas />
</div>
*/

<style>
	.layout {
		position: absolute;
		display: grid;
		grid-template-columns: 300px 1fr;
		height: 100vh;
		width: 100vw;
		color: var(--color-strongest);
		background-color: var(--color-lightest);
	}

	#sidebar {
		display: grid;
		grid-template-rows: auto auto auto auto 1fr 2fr auto;
		gap: 10px;
		max-height: 100vh;
		border-right: 1px solid var(--color-stronger);
		padding: 0 10px;
		background: var(--color-lightest);
	}

	.top-tooling {
		display: grid;
		grid-template-columns: 1fr auto auto;
		align-content: center;
		justify-content: space-between;
		gap: 5px;
	}
</style>
