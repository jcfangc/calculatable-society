import * as THREE from "three";
import { FlyControls } from "three/examples/jsm/controls/FlyControls";
import { createScene } from "./three-cores/createScene";
import { createCamera } from "./three-cores/createCamera";
import { createRenderer } from "./three-cores/createRenderer";
import { createFlyControls } from "./three-cores/createFlyControls";
import { createRenderLoopManager } from "./three-cores/createRenderLoopManager";
import { disposeResources } from "./three-cores/disposeResources";
import { Ref } from "vue";

export interface ThreeCoreOptions {
	container: Ref<HTMLElement>; // 绑定渲染器的 HTML 容器
	width: number; // 相机视图宽度
	height: number; // 相机视图高度
	aspect: number; // 相机的宽高比
	movementSpeed?: number; // FlyControls 的移动速度
	rollSpeed?: number; // FlyControls 的旋转速度
	dragToLook?: boolean; // FlyControls 的鼠标拖动转向开关
	fov?: number; // 相机视野角度
	near?: number; // 相机近裁剪面
	far?: number; // 相机远裁剪面
	rendererOptions?: THREE.WebGLRendererParameters; // 渲染器的配置
	cameraPosition?: THREE.Vector3; // 相机初始位置
	mapDimensions: { rowNum: number; columnNum: number }; // 地图尺寸参数
	checkInterval?: number; // 边界检查间隔帧数
}

export interface ThreeCore {
	scene: THREE.Scene;
	camera: THREE.PerspectiveCamera;
	renderer: THREE.WebGLRenderer;
	controls: FlyControls;
	startRenderLoop: () => void; // 启动渲染循环
	stopRenderLoop: () => void; // 停止渲染循环
	dispose: () => void; // 清理所有资源
}

export class ThreeCoreFactory {
	static createCore(options: ThreeCoreOptions): ThreeCore {
		const {
			container,
			aspect,
			movementSpeed = 10,
			rollSpeed = Math.PI / 5,
			dragToLook = true,
			fov = 75,
			near = 0.1,
			far = 1000,
			rendererOptions = { alpha: true },
			cameraPosition = new THREE.Vector3(0, 0, 5),
			mapDimensions,
			checkInterval = 25,
		} = options;

		const scene = createScene();
		const camera = createCamera(fov, aspect, near, far, cameraPosition);
		const renderer = createRenderer(
			container.value.clientWidth,
			container.value.clientHeight,
			rendererOptions
		);
		container.value.appendChild(renderer.domElement);

		const controls = createFlyControls(
			camera,
			renderer.domElement,
			movementSpeed,
			rollSpeed,
			dragToLook
		);

		// 使用 RenderLoopManager
		const renderLoopManager = createRenderLoopManager();

		const startRenderLoop = () => {
			renderLoopManager.start(
				scene,
				camera,
				renderer,
				controls,
				mapDimensions,
				checkInterval,
				0.02,
				Math.sqrt(3),
				0.1
			);
		};

		const stopRenderLoop = () => {
			renderLoopManager.stop();
		};

		const dispose = () => {
			stopRenderLoop();
			disposeResources(scene, renderer, stopRenderLoop);
		};

		startRenderLoop(); // 启动渲染循环

		return {
			scene,
			camera,
			renderer,
			controls,
			startRenderLoop,
			stopRenderLoop,
			dispose,
		};
	}
}
