import { get, writable } from 'svelte/store';
import { default as init_engine, Engine } from '@drexhage/engine';
import { CanvasDisplay, default as init_ui } from '@drexhage/common-ui';
import { Layer, Tree } from './types';

export enum BlendMode {
	Alpha = 'alpha',
	Remove = 'remove',
	Darken = 'darken',
	Lighten = 'lighten',
	Screen = 'screen'
}

export enum Tool {
	Brush = 'brush',
	Eraser = 'eraser',
	Move = 'move'
}

export enum MovementPolicy {
	Selected = 'selected',
	Clicked = 'clicked'
}

export const ui = writable<CanvasDisplay>();

export const skeleton = writable<boolean>(true);

export const tool = writable(Tool.Brush);

export const radius = writable(30.0);

export const blendMode = writable(BlendMode.Alpha);

export const color1 = writable('#ff0000');

export const color2 = writable('#fee000');

export const hardness = writable(0.5);

export const distance = writable<number>(5);

export const movementPolicy = writable(MovementPolicy.Clicked);

export const alpha = writable(255);

export const layers = writable<Tree<Layer>>(undefined);

export const history = writable<Tree<any>>(undefined);

export const focused = writable<number[]>([]);

export const updatePreview = writable<boolean>(true);

export const current = writable<number>(0);

export const draggingLayers = writable<number | null>(null);

export const undoable = writable<boolean>(false);

export const redoable = writable<boolean>(false);

export const blender = writable<String>('');

async function createSession() {
	// state

	let engine: Engine;
	let wasm;
	let width, height;

	let line_started = false;
	let redraw_callbacks: any[] = [];
	let prev;

	function update() {
		layers.set(engine.content);
		history.set(engine.history);
		current.set(engine.current);
		redoable.set(engine.redoable);
		undoable.set(engine.undoable);
		for (const cb of redraw_callbacks) {
			cb();
		}
	}

	return {
		init: async function (w: number, h: number) {
			await init_ui();
			wasm = await init_engine();
			width = w;
			height = h;
			engine = new Engine(w, h);

			engine.perform_step({
				type: 'layer/create/empty',
				color: '#ffffffff'
			});
			engine.perform_step({
				type: 'layer/create/empty'
			});

			let idx = engine.context_idx;
			if (idx) {
				focused.set([idx]);
			}
			blender.set(engine.blender);
			update();
		},
		switch_blender: function (type) {
			engine.switch_blender(type);
			blender.set(engine.blender);
		},
		add_redraw_callback: function (cb) {
			redraw_callbacks.push(cb);
		},
		move_layer_up: function (idx: number) {
			engine.move_layer_up(idx);
			update();
		},
		move_layer_down: function (idx: number) {
			engine.move_layer_down(idx);
			update();
		},
		perform_step: function (step: any) {
			let before = Date.now();
			engine.perform_step(step);
			let idx = engine.context_idx;
			if (idx) {
				focused.set([idx]);
			}
			update();
			let after = Date.now();
		},
		get_content_of: function (ptr, width, height) {
			return new Uint8ClampedArray(wasm.memory.buffer, ptr, 4 * width * height);
		},
		content_as_bytes: function () {
			// pointing directly into the wasm soup
			return new Uint8ClampedArray(wasm.memory.buffer, engine.pointer, 4 * width * height);
		},
		start: function (x: number, y: number) {
			if (
				get(focused).length !== 1 &&
				!(Tool.Move === get(tool) && get(movementPolicy) === MovementPolicy.Clicked)
			)
				return;
			let step: any = undefined;
			switch (get(tool)) {
				case Tool.Brush:
					step = {
						type: 'draw/line',
						id: get(focused)[0],
						color: get(color1) + get(alpha).toString(16).padStart(2, '0'),
						radius: get(radius),
						mode: get(blendMode),
						hardness: get(hardness),
						distance: get(distance),
						track: []
					};
					break;
				case Tool.Eraser:
					step = {
						type: 'draw/line',
						id: get(focused)[0],
						color: '#000000ff',
						radius: get(radius),
						mode: BlendMode.Remove,
						hardness: get(hardness),
						distance: get(distance),
						track: []
					};
					break;
				case Tool.Move:
					let id = get(focused)[0];
					if (get(movementPolicy) === MovementPolicy.Clicked) {
						prev = get(focused);
						id = engine.get_first_hit(x, y) || id;
						if (!id) return;
						focused.set([id]);
					}
					step = {
						type: 'layer/move_relative',
						id,
						delta: [0, 0]
					};
					break;
			}
			engine.start_step(step);
			line_started = true;
		},
		extend: function (x: number, y: number, xPrev: number, yPrev: number) {
			if (!line_started) return;
			if (get(tool) === Tool.Move) {
				x = Math.ceil(x - xPrev);
				y = Math.ceil(y - yPrev);
			}
			engine.extend_step(x, y);
		},
		finish: function () {
			if (!line_started) return;
			engine.finish_step();
			line_started = false;
			if (prev) {
				focused.set(prev);
				prev = undefined;
			}
			update();
		},
		undo: function () {
			engine.undo();
			update();
		},
		redo: function () {
			engine.redo();
			update();
		}
	};
}

export const session = await createSession();
