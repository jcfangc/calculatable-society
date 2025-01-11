import { FlyControls } from "three/examples/jsm/controls/FlyControls";
import * as THREE from "three";

/**
 * 创建 Three.js 控制器
 * @param camera 相机
 * @param renderer 渲染器
 * @returns FlyControls
 */
export function createControls(
	camera: THREE.PerspectiveCamera,
	renderer: THREE.WebGLRenderer,
	movementSpeed: number = 10,
	rollSpeed: number = Math.PI / 5,
	dragToLook: boolean = true
): FlyControls {
	const controls = new FlyControls(camera, renderer.domElement);
	controls.movementSpeed = movementSpeed;
	controls.rollSpeed = rollSpeed;
	controls.dragToLook = dragToLook;

	return controls;
}
