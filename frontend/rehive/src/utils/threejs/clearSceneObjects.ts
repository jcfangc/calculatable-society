import * as THREE from "three";

/**
 * 清理场景中的所有子对象
 * @param scene THREE.Scene
 */
export function clearSceneObjects(scene: THREE.Scene) {
	while (scene.children.length > 0) {
		scene.remove(scene.children[0]);
	}
}
