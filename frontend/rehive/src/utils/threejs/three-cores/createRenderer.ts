import * as THREE from "three";

/**
 * 创建渲染器
 */
export function createRenderer(
	width: number,
	height: number,
	options: THREE.WebGLRendererParameters
): THREE.WebGLRenderer {
	const renderer = new THREE.WebGLRenderer(options);
	renderer.setSize(width, height);
	return renderer;
}
