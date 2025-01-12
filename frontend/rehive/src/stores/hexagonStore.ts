import { defineStore } from "pinia";

// 定义 Pinia Store
export const useHexagonStore = defineStore("hexagon", () => {
	// 定义 xy 基向量
	const baseVectors = {
		xBaseVector: { x: Math.sqrt(3) * 0.5, y: 0.5 },
		yBaseVector: { x: 0.0, y: 1.0 },
	};

	/**
	 * 根据行列计算六边形的中心坐标
	 * @param {number} row 行号
	 * @param {number} col 列号
	 * @returns {{ x: number, y: number }} 中心坐标
	 */
	function calculateHexagonCenter(row: number, col: number) {
		const centerX =
			col * baseVectors.xBaseVector.x + row * baseVectors.yBaseVector.x;
		const centerY =
			col * baseVectors.xBaseVector.y + row * baseVectors.yBaseVector.y;
		return { x: centerX, y: centerY };
	}

	// 返回全局状态和方法
	return {
		baseVectors,
		calculateHexagonCenter,
	};
});
