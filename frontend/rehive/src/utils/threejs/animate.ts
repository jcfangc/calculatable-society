import * as THREE from "three";
import { FlyControls } from "three/examples/jsm/controls/FlyControls";

/**
 * 动画循环
 * @param scene THREE.Scene
 * @param camera THREE.PerspectiveCamera
 * @param renderer THREE.WebGLRenderer
 * @param controls FlyControls
 */
export function animate(
	scene: THREE.Scene,
	camera: THREE.PerspectiveCamera,
	renderer: THREE.WebGLRenderer,
	controls: FlyControls
) {
	function loop() {
		requestAnimationFrame(loop);
		controls.update(0.02); // 更新控制器状态
		renderer.render(scene, camera);
	}
	loop();
}
