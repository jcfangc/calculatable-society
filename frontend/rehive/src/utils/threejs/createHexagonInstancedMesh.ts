import * as THREE from "three";

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

/**
 * 创建一个六边形的几何体和材质
 * @param radius 六边形的半径
 * @param fillColor 六边形的填充颜色
 * @param edgeColor 六边形的边框颜色
 * @returns 包含六边形几何体和材质的对象
 */
export function createHexagonGeometry(
	radius: number = Math.sqrt(3) / 3,
	fillColor: THREE.ColorRepresentation = rgbToColor(22, 0, 95),
	edgeColor: THREE.ColorRepresentation = rgbToColor(253, 48, 229)
): {
	geometry: THREE.BufferGeometry;
	material: THREE.MeshStandardMaterial;
	edgeMaterial: THREE.LineBasicMaterial;
} {
	// 创建六边形几何体
	const hexagonGeometry = new THREE.CircleGeometry(radius, 6);

	// 创建自发光填充材质
	const material = new THREE.MeshStandardMaterial({
		color: fillColor,
		emissive: fillColor, // 设置自发光颜色
		emissiveIntensity: 1.0, // 自发光强度
		vertexColors: true, // 启用顶点颜色
	});

	// 创建边框材质
	const edgeMaterial = new THREE.LineBasicMaterial({
		color: edgeColor,
		vertexColors: true,
	});

	return { geometry: hexagonGeometry, material, edgeMaterial };
}

/**
 * 创建一个六边形矩阵，使用 InstancedMesh 优化
 * @param rowNum 行数
 * @param columnNum 列数
 * @param hexagonSize 六边形的大小
 * @returns 一个包含六边形的 InstancedMesh 对象
 */
export const createHexagonMatrixInstanced = (
	rowNum: number,
	columnNum: number,
	hexagonSize: number = 0.5
): THREE.InstancedMesh => {
	// 基础六边形的几何体和材质
	const { geometry, material } = createHexagonGeometry(hexagonSize);

	// 启用顶点颜色
	material.vertexColors = true;

	// 计算六边形的数量
	const instanceCount = rowNum * columnNum;

	// 创建 InstancedMesh
	const instancedMesh = new THREE.InstancedMesh(
		geometry,
		material,
		instanceCount
	);

	// 基础向量用于计算每个六边形的位置(立方体坐标=>笛卡尔坐标)
	const xBaseVector = {
		x: Math.sqrt(3) * 0.5,
		y: 0.5,
	};
	const yBaseVector = { x: 0.0, y: 1 };

	// 设置每个实例的位置和颜色
	const dummy = new THREE.Object3D();
	let index = 0;
	for (let row = 0; row < rowNum; row++) {
		for (let col = 0; col < columnNum; col++) {
			// 计算六边形的中心位置
			const centerX = col * xBaseVector.x + row * yBaseVector.x;
			const centerY = col * xBaseVector.y + row * yBaseVector.y;

			// 设置实例的位置
			dummy.position.set(centerX, centerY, 0);
			dummy.updateMatrix(); // 更新实例矩阵
			instancedMesh.setMatrixAt(index, dummy.matrix);

			// 设置实例的颜色（可选：根据密度或其他属性动态调整颜色）
			const color = new THREE.Color(
				Math.random(), // R
				Math.random(), // G
				Math.random() // B
			);
			instancedMesh.setColorAt(index, color);

			index++;
		}
	}

	// 标记需要更新实例矩阵和颜色
	instancedMesh.instanceMatrix.needsUpdate = true;
	// 标记需要更新实例颜色
	if (instancedMesh.instanceColor) {
		instancedMesh.instanceColor.needsUpdate = true; // 确保 instanceColor 存在
	}

	return instancedMesh;
};
