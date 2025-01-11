import * as THREE from "three";

/**
 * 创建 Three.js 场景和相机
 * @param aspect 相机的宽高比
 * @returns { scene: THREE.Scene, camera: THREE.PerspectiveCamera }
 */
export function createSceneAndCamera(aspect: number) {
	const scene = new THREE.Scene();
	const camera = new THREE.PerspectiveCamera(75, aspect, 0.1, 250);
	camera.position.z = 5;
	return { scene, camera };
}
