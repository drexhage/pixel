// Needed for running the wasm part in dev mode

// if you want to generate a static html file
// for your page.
export const prerender = true;

// if you want to Generate a SPA
// you have to set ssr to false.

export const ssr = false; // this must be false for vis.js

// the URL for about page witll be /about with 'ignore' (default)
// the URL for about page witll be /about/ with 'always'
// https://kit.svelte.dev/docs/page-options#trailingslash
export const trailingSlash = 'never';
