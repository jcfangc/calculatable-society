import * as THREE from "three";

/**
 * 清理资源
 */
export function disposeResources(
	scene: THREE.Scene,
	renderer: THREE.WebGLRenderer,
	stopRenderLoop: () => void
) {
	stopRenderLoop();
	scene.children.forEach((child) => {
		if (child instanceof THREE.Mesh) {
			child.geometry.dispose();
			if (child.material instanceof THREE.Material) {
				child.material.dispose();
			}
		}
		scene.remove(child);
	});
	renderer.domElement.remove();
	renderer.dispose();
}
