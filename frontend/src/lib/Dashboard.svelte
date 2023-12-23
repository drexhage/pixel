<script>
	import { skeleton } from '../store';
	import Button from './generic/Button.svelte';
	import Card from './generic/Card.svelte';
	import VisualSwitch from './generic/VisualSwitch.svelte';
	import ButtonColorScheme from './generic/ButtonColorScheme.svelte';
	import { base } from '$app/paths';
	import { onMount } from 'svelte';

	skeleton.set(false);

	function newProject(w, h) {
		return () => {
			if (orientation == 'vertical') {
				let temp = w;
				w = h;
				h = temp;
			}
			location.href = `${base}/editor?w=${w}&h=${h}`;
		};
	}

	function openLocalProject(title) {
		return () => {
			location.href = `${base}/editor?local=${title}`;
		};
	}

	let locals = [];

	onMount(() => {
		for (var i = 0, len = localStorage.length; i < len; ++i) {
			let key = localStorage.key(i);
			if (!key.startsWith('local/')) continue;
			let localSession = JSON.parse(localStorage.getItem(key));
			locals.push({
				title: key.split('/')[1],
				preview: localSession['preview']
			});
		}
		locals = locals;
	});

	let sizes = [
		{
			title: 'instagram',
			w: 1080,
			h: 1080
		},
		{
			title: 'Full HD',
			w: 1920,
			h: 1080
		},
		{
			title: 'HD',
			w: 1280,
			h: 720
		},
		{
			title: '2K',
			w: 2048,
			h: 1080
		},
		{
			title: '4K',
			w: 4096,
			h: 2160
		}
	];
	let orientation = 'horizontal';
</script>

<main>
	<Card>
		<div class="content">
			<div class="new-title">
				<h1>New project</h1>
				<ButtonColorScheme />
				<VisualSwitch
					value={orientation}
					onChange={(x) => (orientation = x)}
					options={[
						{
							title: 'vertical',
							icon: 'fa-ruler-vertical',
							shortcut: undefined,
							value: 'vertical'
						},
						{
							title: 'horizontal',
							icon: 'fa-ruler-horizontal',
							shortcut: undefined,
							value: 'horizontal'
						}
					]}
				/>
			</div>
			<div class="create-cards">
				{#each locals as local}
					<Button title={local.title} on:click={openLocalProject(local.title)}>
						<span class="size">{local.title}</span><br />
						<span class="title">local project</span>
					</Button>
				{/each}
				{#each sizes as size}
					<Button title={size.title} on:click={newProject(size.w, size.h)}>
						<span class="size"
							>{orientation == 'horizontal' ? size.w : size.h}x{orientation == 'horizontal'
								? size.h
								: size.w}</span
						><br />
						<span class="title">{size.title}</span>
					</Button>
				{/each}
			</div>
		</div>
	</Card>
</main>

<style>
	main {
		max-width: 600px;
		margin: 20px auto;
	}
	.size {
		font-size: large;
		font-weight: bolder;
	}
	.title {
		font-size: smaller;
	}
	.content {
		padding: 10px;
	}
	.create-cards {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
	}
	.create-cards :global(.btn) {
		flex: 1 1 30%;
		height: 100%;
		padding: 20px;
	}
	.new-title {
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		gap: 10px;
		align-items: center;
	}
</style>
