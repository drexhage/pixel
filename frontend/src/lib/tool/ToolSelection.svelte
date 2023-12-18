<script lang="ts">
	import { Tool, tool } from '../../store';
	import ColorPicker from './ColorPicker.svelte';
	import VisualSwitch from '../generic/VisualSwitch.svelte';

	let tools: { value: any; icon: string; title: string; shortcut: any }[] = [
		{
			value: Tool.Brush,
			icon: 'fa-pencil',
			title: 'brush',
			shortcut: { shift: true, code: 'Digit1' }
		},
		{
			value: Tool.Eraser,
			icon: 'fa-eraser',
			title: 'eraser',
			shortcut: { shift: true, code: 'Digit2' }
		},
		{
			value: Tool.Move,
			icon: 'fa-arrows',
			title: 'move',
			shortcut: { shift: true, code: 'Digit3' }
		}
	];
</script>

<div id="tools">
	<ColorPicker />
	<VisualSwitch value={$tool} options={tools} onChange={(e) => tool.set(e)} />
</div>

<style>
	#tools {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 10px;
	}
</style>
