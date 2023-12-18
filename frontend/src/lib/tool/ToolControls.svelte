<script lang="ts">
	import {
		BlendMode,
		Tool,
		radius,
		tool,
		blendMode,
		alpha,
		MovementPolicy,
		movementPolicy,
		session,
		focused,
		hardness,
		distance
	} from '../../store';
	import Button from '../generic/Button.svelte';
	import ButtonGroup from '../generic/ButtonGroup.svelte';
	import Selection from '../generic/Selection.svelte';
	import Slider from '../generic/Slider.svelte';
	import BrushPreview from './BrushPreview.svelte';

	const options = [
		{
			value: MovementPolicy.Selected,
			label: 'Move selected layer'
		},
		{
			value: MovementPolicy.Clicked,
			label: 'Move layer that is clicked'
		}
	];

	function flip(direction) {
		return () => {
			if ($focused.length == 0) return;
			session.perform_step({
				type: 'layer/flip',
				id: $focused[0],
				direction
			});
		};
	}

	let oneFocused = $focused.length == 1;
	$: oneFocused = $focused.length == 1;
</script>

<div class="box">
	{#if $tool === Tool.Brush || $tool === Tool.Eraser}
		<div class="compact">
			<BrushPreview width={60} height={60} />
			<div class="whatever">
				<Slider
					min={0}
					max={255}
					bind:value={$alpha}
					name="alpha"
					fmt={(x) => `${~~((x / 255) * 100)}%`}
				/>
				<Slider
					min={0}
					max={1}
					step={0.01}
					bind:value={$hardness}
					name="hardness"
					fmt={(x) => `${~~(x * 100)}%`}
				/>
			</div>
		</div>
		<Slider min={0} max={100} bind:value={$radius} name="radius" fmt={(x) => `${x}px`} />
		<Slider min={1} max={100} step={1} bind:value={$distance} name="distance" fmt={(x) => `${x}`} />
		{#if $tool === Tool.Brush}
			<Selection
				title="mode"
				bind:value={$blendMode}
				options={[
					{ key: null, value: BlendMode.Alpha },
					{ key: null, value: BlendMode.Darken },
					{ key: null, value: BlendMode.Lighten },
					{ key: null, value: BlendMode.Screen }
				]}
			/>
		{/if}
	{:else if $tool === Tool.Move}
		<ButtonGroup>
			<Button on:click={flip('vertically')} disabled={!oneFocused} title="flip vertically"
				><i class="fa-solid fa-up-down" /></Button
			>
			<Button on:click={flip('horizontally')} disabled={!oneFocused} title="flip horizontally"
				><i class="fa-solid fa-left-right" /></Button
			>
		</ButtonGroup>
		<ul>
			{#each options as { value, label }}
				<li>
					<input type="radio" bind:group={$movementPolicy} {value} />{label}
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.box {
		display: flex;
		flex-direction: column;
		gap: 10px;
		padding: 15px;
	}

	.compact {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 10px;
	}
	.whatever {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}
</style>
