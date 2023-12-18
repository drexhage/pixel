<script lang="ts">
	import { onMount } from 'svelte';

	import { draggingLayers, focused, session } from '../../store';
	import type { Node, Layer } from '../../types';
	import LayerList from './LayerList.svelte';
	import DragDropIndicator from './DragDropIndicator.svelte';
	import LevelPadding from './LevelPadding.svelte';

	export let node: Node<Layer>;
	export let level: number;

	let hovered = false;
	let collapse = true;
	let locked = false;

	let dragged = false;

	let preview: HTMLImageElement;
	let [width, height] = node.value.size;
	let preview_content = session.get_content_of(node.value.pointer, width, height);
	let image_data = new ImageData(preview_content, width, height);

	onMount(async () => {
		if (!preview || !image_data) return;
		let canvas = document.createElement('canvas');
		canvas.width = width;
		canvas.height = height;
		let ctx = canvas.getContext('2d')!;
		ctx.putImageData(image_data, 0, 0);
		preview.src = canvas.toDataURL('image/png');
	});

	function focus(e: MouseEvent) {
		let add = e.shiftKey;
		let alreadySelected = $focused.includes(node.id);

		if (add) {
			if (alreadySelected) {
				focused.update((current) => current.filter((x) => x !== node.id));
			} else {
				focused.update((current) => current.concat([node.id]));
			}
		} else {
			focused.set([node.id]);
		}
	}

	function toggleVisible() {
		session.perform_step({
			type: 'layer/attr',
			id: node.id,
			visible: !node.value.visible
		});
	}

	function toggleLocked() {
		locked = !locked;
	}

	function toggleCollapse() {
		collapse = !collapse;
	}

	function hoverCb(hover: boolean) {
		return () => {
			hovered = hover;
		};
	}

	function onInput(e) {
		if (e.inputType === 'insertParagraph' || (e.inputType === 'insertText' && !e.data)) {
			e.stopPropagation();
			let name: string = e.target['innerText'];
			name = name.replace('\n', '');
			name = name.replace('\r', '');
			e.target['innerText'] = name;
			if (name != node.value.name) {
				session.perform_step({
					type: 'layer/attr',
					id: node.id,
					name: name
				});
			}
			return;
		}
	}

	function drag(start: boolean) {
		return () => {
			dragged = start;
			if (start) {
				draggingLayers.set(node.id);
			} else {
				draggingLayers.set(null);
			}
		};
	}

	let selected = false;
	$: selected = $focused.includes(node.id);
</script>

<li class="layers" style:background={selected ? 'var(--color-lighter)' : 'var(--color-lightest)'}>
	<div
		class="layer-box"
		on:click={focus}
		on:keydown
		on:mouseenter={hoverCb(true)}
		on:mouseleave={hoverCb(false)}
		on:dragstart={drag(true)}
		on:dragend={drag(false)}
		draggable="true"
	>
		<LevelPadding {level} />
		<div class="layer-info">
			<div class="left-of-preview" on:click={toggleCollapse} on:keydown>
				{#if node.value.flag.type === 'Group'}
					{#if collapse}
						<i class="fa fa-chevron-down" />
					{:else}
						<i class="fa fa-chevron-right" />
					{/if}
				{/if}
			</div>
			<img class="preview" bind:this={preview} alt={'preview of layer'} />
			<div class="right-of-preview">
				<span class="layer-name" on:input={onInput} spellcheck="false" contenteditable={selected}
					>{node.value.name}</span
				>
				<div>
					{#if hovered || !node.value.visible}
						<button class="visibility" on:click={toggleVisible}>
							{#if node.value.visible}
								<i class="fa fa-eye" />
							{:else}
								<i class="fa fa-eye-slash" />
							{/if}</button
						>
					{/if}
				</div>
			</div>
		</div>
	</div>
	{#if node.value.flag.type === 'Group' && collapse}
		<LayerList children={node.children} level={level + 1} move_idx={-node.id} />
	{/if}
	<DragDropIndicator move_idx={node.id} {level} />
</li>

<style>
	.preview {
		max-height: 50px;
		max-width: 50px;
	}
	button.visibility {
		border: none;
		outline: none;
		background: none;
		color: var(--color-stronger);
		&:hover {
			cursor: pointer;
		}
	}

	.left-of-preview {
		width: 25px;
	}

	.right-of-preview {
		display: flex;
		padding: 2px;
		margin-left: 10px;
		justify-content: space-between;
	}

	.layer-box {
		display: grid;
		grid-template-columns: auto 1fr;
		align-items: center;
		height: 65px;
		&:hover {
			background-color: var(--color-light);
		}
	}

	div.layer-info {
		display: grid;
		grid-template-columns: 20px auto 1fr;
		align-items: center;
		padding: 5px;
	}

	li.layers {
		user-select: none;
		width: 100%;
		padding: 0;
		box-sizing: border-box;
		&:hover {
			cursor: pointer;
		}
	}

	img {
		border: 1px dashed var(--color-strongest);
		box-sizing: border-box;
	}
</style>
