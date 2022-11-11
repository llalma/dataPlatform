import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const config: UserConfig = {
	server: {
		fs: {
		  // Allow serving files from one level up to the project root
		  allow: ['..']
		}
	  },
	plugins: [
		wasm(),
		topLevelAwait(),
		sveltekit()]
};

export default config;