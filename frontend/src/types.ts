export interface Node<T> {
	id: number;
	value: T;
	children: number[]; // Object.fromEntries(value) to retrieve Node<T>
	parent: number;
}

export interface Tree<T> {
	root: number;
	nodes: Node<T>[];
}

export interface Layer {
	attr: {
		mode: string;
		alpha: number;
		pos: [number, number];
	};
	flag: { type: string };
	size: [number, number];
	pointer: number;
	visible: boolean;
	name: string;
}

export interface Step {
	data: {
		type: string;
	};
}
