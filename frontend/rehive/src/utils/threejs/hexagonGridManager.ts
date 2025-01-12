import * as THREE from "three";
import { useHexagonStore } from "@/stores/hexagonStore";

/**
 * 配置接口，用于定义网格和相机相关参数
 */
export interface HexagonGridConfig {
	rowNum: number; // 行数
	columnNum: number; // 列数
	hexagonSize?: number; // 六边形大小，默认为 0.5
	colorFn?: (row: number, col: number) => THREE.Color; // 可选的颜色生成函数
	defaultColors?: [
		[number, number, number], // 填充颜色
		[number, number, number] // 边框颜色
	];
	scene: THREE.Scene; // 必须提供的场景对象
	camera: THREE.PerspectiveCamera; // 必须提供的相机对象
}

export class HexagonGridManager {
	/**
	 * 创建一个六边形矩阵网格，使用 InstancedMesh 优化
	 * @param config HexagonGridConfig 配置对象
	 * @returns 包含六边形的 InstancedMesh 对象
	 */
	static createHexagonGrid(config: HexagonGridConfig): THREE.InstancedMesh {
		const {
			rowNum,
			columnNum,
			hexagonSize = 0.5,
			colorFn,
			defaultColors = [
				[22, 0, 95], // 默认填充颜色
				[253, 48, 229], // 默认边框颜色
			],
			scene,
		} = config;

		if (!scene) {
			throw new Error(
				"The 'scene' parameter is required and cannot be null."
			);
		}

		const fillColor = rgbToColor(...defaultColors[0]);
		const edgeColor = rgbToColor(...defaultColors[1]);

		const material = new THREE.MeshStandardMaterial({
			color: fillColor,
			emissive: edgeColor,
			emissiveIntensity: 1.0,
			vertexColors: !!colorFn,
		});

		const geometry = new THREE.CircleGeometry(hexagonSize, 6);
		const instanceCount = rowNum * columnNum;
		const instancedMesh = new THREE.InstancedMesh(
			geometry,
			material,
			instanceCount
		);

		const { calculateHexagonCenter } = useHexagonStore();

		const dummy = new THREE.Object3D();
		let index = 0;

		for (let row = 0; row < rowNum; row++) {
			for (let col = 0; col < columnNum; col++) {
				const { x: centerX, y: centerY } = calculateHexagonCenter(
					row,
					col
				);

				dummy.position.set(centerX, centerY, 0);
				dummy.updateMatrix();
				instancedMesh.setMatrixAt(index, dummy.matrix);

				if (colorFn) {
					const color = colorFn(row, col);
					instancedMesh.setColorAt(index, color);
				}

				index++;
			}
		}

		instancedMesh.instanceMatrix.needsUpdate = true;
		if (colorFn && instancedMesh.instanceColor) {
			instancedMesh.instanceColor.needsUpdate = true;
		}

		scene.add(instancedMesh);

		return instancedMesh;
	}

	/**
	 * 根据 InstancedMesh 和配置自动设置相机默认位置和视角
	 * @param config HexagonGridConfig 配置对象
	 * @param instancedMesh 生成的 InstancedMesh 对象
	 */
	static calculateDefaultCameraPosition(
		config: HexagonGridConfig,
		instancedMesh: THREE.InstancedMesh
	): { defaultPosition: THREE.Vector3; defaultLookAt: THREE.Vector3 } {
		const { rowNum, columnNum, camera } = config;
		const dummy = new THREE.Object3D();

		// 获取第一个实例的矩阵
		const firstMatrix = new THREE.Matrix4();
		instancedMesh.getMatrixAt(0, firstMatrix);
		dummy.applyMatrix4(firstMatrix);
		const firstPosition = dummy.position.clone();

		// 获取最后一个实例的矩阵
		const lastMatrix = new THREE.Matrix4();
		instancedMesh.getMatrixAt(instancedMesh.count - 1, lastMatrix);
		dummy.applyMatrix4(lastMatrix);
		const lastPosition = dummy.position.clone();

		// 计算全局中心坐标
		const globalCenterX = (firstPosition.x + lastPosition.x) / 2;
		const globalCenterY = (firstPosition.y + lastPosition.y) / 2;

		// 计算相机默认位置
		const defaultPosition = new THREE.Vector3(
			globalCenterX,
			globalCenterY,
			Math.min(Math.max(rowNum, columnNum), 25)
		);

		// 设置相机位置和视角
		camera.position.set(
			defaultPosition.x,
			defaultPosition.y,
			defaultPosition.z
		);

		const defaultLookAt = new THREE.Vector3(
			globalCenterX,
			globalCenterY,
			0
		);
		camera.lookAt(defaultLookAt);

		return { defaultPosition, defaultLookAt };
	}
}

/**
 * 将 RGB 值从 0-255 映射到 0-1
 * @param r 红色通道值 (0-255)
 * @param g 绿色通道值 (0-255)
 * @param b 蓝色通道值 (0-255)
 * @returns THREE.Color
 */
export function rgbToColor(r: number, g: number, b: number): THREE.Color {
	return new THREE.Color(r / 255, g / 255, b / 255);
}
