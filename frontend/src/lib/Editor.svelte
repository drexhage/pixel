<script lang="ts">
	import { CanvasDisplay } from '@drexhage/common-ui';
	import { session, skeleton, ui } from '../store';
	import { colorScheme, startListen } from '../color';
	import Desktop from '../lib/layout/Desktop.svelte';
	import { onMount } from 'svelte';

	$: localStorage.setItem('colorscheme', $colorScheme);
	const delay = (ms) => new Promise((res) => setTimeout(res, ms));
	const urlParams = new URLSearchParams(window.location.search);

	async function loadUrlAsLayer(url) {
		return new Promise((resolve, reject) => {
			let image = new Image();
			image.onload = (e) => {
				let data = e.target['result'];
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
			image.src = url;
		});
	}

	let init = async () => {
		let w = parseInt(urlParams.get('w')) || 2000;
		let h = parseInt(urlParams.get('h')) || 1500;
		urlParams.delete('w');
		urlParams.delete('h');
		await delay(500);
		await session.init(w, h);
		skeleton.set(false);
		ui.set(new CanvasDisplay(w, h, 100, 100, 10));
	};
	onMount(async () => {
		init();
		startListen();
	});
</script>

<Desktop />
