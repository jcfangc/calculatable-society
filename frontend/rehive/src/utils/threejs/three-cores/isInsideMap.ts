import * as THREE from "three";

/**
 * 检查点是否在地图范围内
 * @param camera 相机实例
 * @param config 地图配置，包括行列数和允许的超出值
 * @param sqrt3 根号3常量
 * @returns 是否在地图范围内
 */
export function isInsideMap(
	camera: THREE.PerspectiveCamera,
	config: { rowNum: number; columnNum: number; exceeding: number },
	sqrt3: number
): boolean {
	// 地图边界计算
	const halfSqrt3 = sqrt3 / 2;

	// 检查 X 和 Y 的边界
	const x = camera.position.x;
	const y = camera.position.y;
	const z = camera.position.z;

	return (
		x >= 0 &&
		x <= halfSqrt3 * config.columnNum &&
		y >= x / sqrt3 &&
		y <= config.rowNum + x / sqrt3 &&
		z >= config.exceeding
	);
}
