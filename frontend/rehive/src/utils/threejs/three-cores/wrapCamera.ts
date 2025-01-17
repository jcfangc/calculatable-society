import * as THREE from "three";

/**
 * 处理相机的边界逻辑
 */
export function wrapCameraPosition(
	camera: THREE.PerspectiveCamera,
	config: { rowNum: number; columnNum: number; exceeding: number },
	sqrt3: number,
	epsilon: number
): void {
	const makeUp = config.exceeding * epsilon;

	// 避免 z 轴超出
	if (camera.position.z < config.exceeding) {
		camera.position.z = config.exceeding;
	}

	// 环绕处理
	if (camera.position.x > (sqrt3 / 2) * config.columnNum) {
		camera.position.x -= (sqrt3 / 2) * config.columnNum - makeUp;
		camera.position.y -= 0.5 * config.columnNum - makeUp;
	} else if (camera.position.x < 0) {
		camera.position.x += (sqrt3 / 2) * config.columnNum - makeUp;
		camera.position.y += 0.5 * config.columnNum - makeUp;
	}

	if (camera.position.y > config.rowNum + camera.position.x / sqrt3) {
		camera.position.y -= config.rowNum - makeUp;
	} else if (camera.position.y < camera.position.x / sqrt3) {
		camera.position.y += config.rowNum - makeUp;
	}
}
