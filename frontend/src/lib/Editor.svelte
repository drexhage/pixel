<script>
	import { current, history, session, skeleton } from '../store';
	import Desktop from '../lib/layout/Desktop.svelte';
	import { onMount } from 'svelte';
	import { base } from '$app/paths';

	const delay = (ms) => new Promise((res) => setTimeout(res, ms));
	const urlObj = new URL(window.location.toString());

	function moveHome() {
		let session = { idx: $current, history: $history, preview: null };
		let key = `local/default`;
		let item = localStorage.getItem(key);
		if (item) {
			if (confirm('Replace stored local project with the current one?')) {
				localStorage.setItem(key, JSON.stringify(session));
			}
		}
		window.location = `${base}/`;
	}

	// save editing history when leaving
	window.onbeforeunload = function () {
		return 'Are you sure?';
	};

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
		let w = parseInt(urlObj.searchParams.get('w')) || 2000;
		let h = parseInt(urlObj.searchParams.get('h')) || 1500;
		let source = urlObj.searchParams.get('local');
		if (source) {
			let localSession = JSON.parse(localStorage.getItem(`local/${source}`));
			if (localSession) {
				await session.from_history(localSession.idx, localSession.history);
			} else {
				await session.init(w, h);
			}
		} else {
			urlObj.searchParams.delete('w');
			urlObj.searchParams.delete('h');
			window.history.pushState(undefined, undefined, urlObj.href); // remove search params
			await delay(500);
			await session.init(w, h);
		}
		skeleton.set(false);
	};
	onMount(async () => {
		init();
	});
</script>

<Desktop {moveHome} />
