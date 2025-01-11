import * as THREE from "three";

/**
 * 创建 Three.js 渲染器
 * @param width 渲染器宽度
 * @param height 渲染器高度
 * @returns THREE.WebGLRenderer
 */
export function createRenderer(
	width: number,
	height: number
): THREE.WebGLRenderer {
	const renderer = new THREE.WebGLRenderer({ alpha: true });
	renderer.setSize(width, height);
	return renderer;
}
