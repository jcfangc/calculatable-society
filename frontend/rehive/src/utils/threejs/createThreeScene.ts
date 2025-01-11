// hooks/useThreeScene.hook.ts
import { Ref } from "vue";
import * as THREE from "three";
import { FlyControls } from "three/examples/jsm/controls/FlyControls";
import {
	createSceneAndCamera,
	createRenderer,
	clearSceneObjects,
	animate,
	createHexagonMatrixInstanced,
	createControls,
	createCameraController,
} from "@/utils/threejs";

interface ThreeRefs {
	scene: THREE.Scene | null;
	camera: THREE.PerspectiveCamera | null;
	renderer: THREE.WebGLRenderer | null;
	controls: FlyControls | null;
}

export function makeThreeRefs(
	scene: THREE.Scene | null,
	camera: THREE.PerspectiveCamera | null,
	renderer: THREE.WebGLRenderer | null,
	controls: FlyControls | null
): ThreeRefs {
	return { scene, camera, renderer, controls };
}

/**
 * 将 scene、camera、renderer、controls 从外部传入，这样 hook 本身并不持有它们，
 * 而只是帮你初始化或更新它们
 */
export function createThreeScene(
	canvas: Ref<HTMLDivElement | null>,
	width: number,
	height: number,
	threeRefs: ThreeRefs
) {
	/**
	 * 初始化场景、相机、渲染器以及控制器
	 */
	const initializeScene = () => {
		if (!canvas.value) return;

		const { scene, camera } = createSceneAndCamera(width / height);
		threeRefs.scene = scene;
		threeRefs.camera = camera;

		const renderer = createRenderer(width, height);
		canvas.value.appendChild(renderer.domElement);
		threeRefs.renderer = renderer;

		const controls = createControls(camera, renderer);
		threeRefs.controls = controls;

		if (scene && camera && renderer && controls) {
			animate(scene, camera, renderer, controls);
		}

		// 返回初始化的对象
		return threeRefs;
	};

	// 用来记录网格中心和默认的相机位置
	let globalCenterX: number;
	let globalCenterY: number;
	let defaultPosition: THREE.Vector3;
	let defaultLookAt: THREE.Vector3;

	/**
	 * 根据数据更新场景
	 */
	const updateSceneWithData = (data: any) => {
		const { scene, camera } = threeRefs;

		if (!scene || !camera) return;

		clearSceneObjects(scene);

		// 假设数据为 rowNum 和 columnNum
		const rowNum: number = data.rowNum || 5;
		const columnNum: number = data.columnNum || 5;

		// 调用 createHexagonMatrix
		const hexagonInstancedMesh = createHexagonMatrixInstanced(
			rowNum,
			columnNum
		);

		// 将六边形网格添加到场景
		scene.add(hexagonInstancedMesh);

		// 获取第一个和最后一个实例的位置
		const dummy = new THREE.Object3D(); // 用于读取矩阵数据

		// 获取第一个实例的矩阵
		const firstMatrix = new THREE.Matrix4();
		hexagonInstancedMesh.getMatrixAt(0, firstMatrix); // 索引 0 是第一个实例
		dummy.applyMatrix4(firstMatrix);
		const firstPosition = dummy.position.clone();

		// 获取最后一个实例的矩阵
		const lastMatrix = new THREE.Matrix4();
		hexagonInstancedMesh.getMatrixAt(
			hexagonInstancedMesh.count - 1,
			lastMatrix
		); // 最后一个实例
		dummy.applyMatrix4(lastMatrix);
		const lastPosition = dummy.position.clone();

		// 计算全局中心坐标
		globalCenterX = (firstPosition.x + lastPosition.x) / 2;
		globalCenterY = (firstPosition.y + lastPosition.y) / 2;

		// 调整相机位置
		defaultPosition = new THREE.Vector3(
			globalCenterX,
			globalCenterY,
			Math.min(Math.max(rowNum, columnNum), 25)
		);
		camera.position.set(
			defaultPosition.x,
			defaultPosition.y,
			defaultPosition.z
		);

		defaultLookAt = new THREE.Vector3(globalCenterX, globalCenterY, 0);
		camera.lookAt(defaultLookAt);

		return { defaultPosition, defaultLookAt };
	};

	/**
	 * 更新渲染器尺寸
	 */
	const updateRendererSize = () => {
		const { renderer, camera } = threeRefs;
		if (renderer && camera) {
			renderer.setSize(width, height);
			camera.aspect = width / height;
			camera.updateProjectionMatrix();
		}
	};

	return {
		initializeScene,
		updateSceneWithData,
		updateRendererSize,
	};
}
