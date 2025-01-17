import * as THREE from "three";
import { FlyControls } from "three/examples/jsm/controls/FlyControls";
import { isInsideMap } from "./isInsideMap";
import { wrapCameraPosition } from "./wrapCamera";

export function createRenderLoopManager() {
	let renderLoopActive = false; // 渲染循环的活动状态
	let requestId: number | null = null; // 存储 requestAnimationFrame 的 ID

	return {
		/**
		 * 启动渲染循环
		 */
		start: (
			scene: THREE.Scene,
			camera: THREE.PerspectiveCamera,
			renderer: THREE.WebGLRenderer,
			controls: FlyControls,
			mapDimensions: { rowNum: number; columnNum: number },
			checkInterval: number,
			frameTime: number,
			sqrt3: number,
			epsilon: number
		) => {
			if (renderLoopActive) {
				console.warn("Render loop is already running!");
				return;
			}

			renderLoopActive = true;
			let frameCount = 0;

			const exceeding =
				controls.movementSpeed * checkInterval * frameTime;
			const config = {
				rowNum: mapDimensions.rowNum,
				columnNum: mapDimensions.columnNum,
				exceeding,
			};

			const loop = () => {
				if (!renderLoopActive) return;

				requestId = requestAnimationFrame(loop);

				frameCount++;
				if (frameCount % checkInterval === 0) {
					frameCount = 0;

					if (!isInsideMap(camera, config, sqrt3)) {
						wrapCameraPosition(camera, config, sqrt3, epsilon);
					}
				}

				controls.update(frameTime);
				renderer.render(scene, camera);
			};

			loop();
		},

		/**
		 * 停止渲染循环
		 */
		stop: () => {
			if (!renderLoopActive) {
				console.warn("Render loop is not active!");
				return;
			}

			renderLoopActive = false;

			if (requestId !== null) {
				cancelAnimationFrame(requestId);
				requestId = null;
			}
		},

		/**
		 * 检查渲染循环状态
		 */
		isActive: () => renderLoopActive,
	};
}
