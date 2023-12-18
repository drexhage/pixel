import { writable } from 'svelte/store';

// ignoring system colorscheme right now

export enum ColorScheme {
	Light = 'light',
	Dark = 'dark',
	System = 'system'
}

const retrieveStoredScheme = () => {
	const stored = localStorage.getItem('theme');
	return stored === 'light' ? ColorScheme.Light : ColorScheme.Dark;
};

const isDarkMode = () => {
	return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
};

export const startListen = () => {
	window
		.matchMedia('(prefers-color-scheme: dark)')
		.addEventListener('change', ({ matches }) => systemLightScheme.set(!matches));
};

export const colorScheme = writable<ColorScheme>(retrieveStoredScheme());
colorScheme.subscribe((value) => {
	localStorage.setItem('theme', value);
});

export const systemLightScheme = writable<boolean>(!isDarkMode());

export const iterColorScheme = () => {
	colorScheme.update((x) => {
		switch (x) {
			case ColorScheme.Light:
				return ColorScheme.Dark;
			case ColorScheme.Dark:
			//     return ColorScheme.System;
			case ColorScheme.System:
				return ColorScheme.Light;
		}
	});
};
