import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import wasmPack from 'vite-plugin-wasm-pack';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
	plugins: [
		sveltekit(),
		wasm(),
		topLevelAwait(),
		wasmPack(['../crates/engine', '../crates/common-ui'])
	],
	server: {
		fs: {
			strict: false
		}
	}
});
