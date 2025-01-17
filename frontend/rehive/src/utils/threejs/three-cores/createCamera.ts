import * as THREE from "three";

/**
 * 创建透视相机
 */
export function createCamera(
	fov: number,
	aspect: number,
	near: number,
	far: number,
	position: THREE.Vector3
): THREE.PerspectiveCamera {
	const camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
	camera.position.copy(position);
	return camera;
}
