<script lang="ts">
	import { onMount } from 'svelte';
	import { session, ui } from '../../store';

	export let width: number;
	export let height: number;

	let canvas: HTMLCanvasElement;

	// remember for rendering
	let imageData: ImageData;
	let prog: WebGLProgram;
	let tex: WebGLBuffer;

	function init() {
		const gl = canvas.getContext('webgl2')!;

		// create shaders
		let vertexShaderSrc =
			'attribute vec2 aVertex;' +
			'attribute vec2 aUV;' +
			'varying vec2 vTex;' +
			'uniform vec2 pos;' +
			'void main(void) {' +
			'  gl_Position = vec4(aVertex + pos, 0.0, 1.0);' +
			'  vTex = aUV;' +
			'}';

		let fragmentShaderSrc =
			'precision highp float;' +
			'varying vec2 vTex;' +
			'uniform sampler2D sampler0;' +
			'void main(void){' +
			'  gl_FragColor = texture2D(sampler0, vTex);' +
			'}';

		let vertShaderObj = gl.createShader(gl.VERTEX_SHADER);
		let fragShaderObj = gl.createShader(gl.FRAGMENT_SHADER);
		gl.shaderSource(vertShaderObj, vertexShaderSrc);
		gl.shaderSource(fragShaderObj, fragmentShaderSrc);
		gl.compileShader(vertShaderObj);
		gl.compileShader(fragShaderObj);

		prog = gl.createProgram();
		gl.attachShader(prog, vertShaderObj);
		gl.attachShader(prog, fragShaderObj);

		gl.linkProgram(prog);
		gl.useProgram(prog);

		tex = gl.createTexture();

		session.add_redraw_callback(redraw);
	}

	// redrawing based on the current state of the session and the ui canvas display
	export async function redraw() {
		if (!$ui || !prog || !canvas) return;
		$ui.set_canvas_size(width, height);

		const gl = canvas.getContext('webgl2');
		gl.viewport(0, 0, $ui.canvas_size.width, $ui.canvas_size.height);

		let norm_width = 2 * (($ui.scale * $ui.img_size.width) / $ui.canvas_size.width);
		let norm_height = 2 * (($ui.scale * $ui.img_size.height) / $ui.canvas_size.height);
		let norm_x = 2 * ($ui.position.x / $ui.canvas_size.width) - 1;
		let norm_y = 2 * (1 - $ui.position.y / $ui.canvas_size.height) - 1;

		let norm = [
			norm_x,
			norm_y - norm_height,
			norm_x,
			norm_y,
			norm_x + norm_width,
			norm_y,
			norm_x + norm_width,
			norm_y - norm_height
		];

		let vertexBuff = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuff);
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(norm), gl.STATIC_DRAW);

		let texBuff = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, texBuff);
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 1, 0, 0, 1, 0, 1, 1]), gl.STATIC_DRAW);

		let vloc = gl.getAttribLocation(prog, 'aVertex');
		let tloc = gl.getAttribLocation(prog, 'aUV');
		let clamped;
		try {
			clamped = session.content_as_bytes();
		} catch (e) {
			console.log(e);
			return;
		}
		imageData = new ImageData(clamped, $ui.img_size.width, $ui.img_size.height);

		gl.bindTexture(gl.TEXTURE_2D, tex);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
		gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, imageData);

		gl.enableVertexAttribArray(vloc);
		gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuff);
		gl.vertexAttribPointer(vloc, 2, gl.FLOAT, false, 0, 0);

		gl.enableVertexAttribArray(tloc);
		gl.bindBuffer(gl.ARRAY_BUFFER, texBuff);
		gl.bindTexture(gl.TEXTURE_2D, tex);
		gl.vertexAttribPointer(tloc, 2, gl.FLOAT, false, 0, 0);

		gl.drawArrays(gl.TRIANGLE_FAN, 0, 4);
	}

	function sizeChanged() {
		$ui.set_canvas_size(width, height);
		$ui.center();
		redraw();
	}
	$: width && sizeChanged();
	$: height && sizeChanged();

	onMount(async () => {
		await init();
		await sizeChanged();
	});
</script>

<canvas id="canvas-content" bind:this={canvas} {width} {height} />

<style>
	canvas {
		position: absolute;
		top: 0;
		left: 0;
		display: block;
		width: 100%;
		height: 100%;
		image-rendering: optimizeSpeed;
		image-rendering: -moz-crisp-edges;
		image-rendering: -webkit-optimize-contrast;
		image-rendering: -o-crisp-edges;
		image-rendering: pixelated;
		-ms-interpolation-mode: nearest-neighbor;
	}
</style>
