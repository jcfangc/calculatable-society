import * as THREE from "three";

/**
 * 创建相机控制器
 * @param camera THREE.js 的相机对象
 * @param defaultPosition 相机的默认位置
 * @param defaultLookAt 相机的默认观察点
 * @returns 一组相机操作方法
 */
export function createCameraController(
	camera: THREE.Camera,
	defaultPosition: THREE.Vector3,
	defaultLookAt: THREE.Vector3
) {
	/**
	 * 重置相机到默认位置
	 */
	const resetCameraPosition = () => {
		camera.position.copy(defaultPosition);
		camera.lookAt(defaultLookAt);
		console.log("Camera reset to default position.");
	};

	/**
	 * 摄像头反转 180 度
	 */
	const reverseCamera = () => {
		camera.rotation.y += Math.PI; // 旋转 180 度
		console.log("Camera reversed 180 degrees.");
	};

	/**
	 * 更新摄像机的 Z 坐标
	 * @param zValue 用户输入的 Z 坐标值
	 */
	const updateCameraZ = (zValue: number) => {
		camera.position.z = zValue;
		console.log(`Camera Z position updated to: ${zValue}`);
	};

	// 返回操作方法
	return {
		resetCameraPosition,
		reverseCamera,
		updateCameraZ,
	};
}
