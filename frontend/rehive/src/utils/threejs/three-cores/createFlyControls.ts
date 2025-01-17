import * as THREE from "three";
import { FlyControls } from "three/examples/jsm/controls/FlyControls";

/**
 * 创建 FlyControls
 */
export function createFlyControls(
	camera: THREE.PerspectiveCamera,
	domElement: HTMLElement,
	movementSpeed: number,
	rollSpeed: number,
	dragToLook: boolean
): FlyControls {
	const controls = new FlyControls(camera, domElement);
	controls.movementSpeed = movementSpeed;
	controls.rollSpeed = rollSpeed;
	controls.dragToLook = dragToLook;
	return controls;
}
