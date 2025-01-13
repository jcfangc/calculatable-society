import * as THREE from "three";
import { FlyControls } from "three/examples/jsm/controls/FlyControls";
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
	private static renderLoopActive = false;
	private static sqrt3 = Math.sqrt(3);
	private static halfSqrt3 = this.sqrt3 / 2;
	private static epsilon = 0.1;

	/**
	 * 创建所有核心元素并集成 Vue 3 生命周期管理
	 * @param options 配置项
	 * @returns 包含所有核心元素和资源管理方法的对象
	 */
	static createCore(options: ThreeCoreOptions): ThreeCore {
		// 解构并提供默认值
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

		// 创建核心元素
		const scene = this.createScene();
		const camera = this.createPerspectiveCamera(
			fov,
			aspect,
			near,
			far,
			cameraPosition
		);
		const renderer = this.createRenderer(
			container.value.clientWidth,
			container.value.clientHeight,
			rendererOptions
		);
		container.value.appendChild(renderer.domElement);

		const controls = this.createFlyControls(
			camera,
			renderer.domElement,
			movementSpeed,
			rollSpeed,
			dragToLook
		);

		// 创建渲染循环
		const startRenderLoop = this.startRenderLoop.bind(
			this,
			scene,
			camera,
			renderer,
			controls,
			mapDimensions,
			checkInterval
		);
		const stopRenderLoop = this.stopRenderLoop.bind(this);

		// 创建资源释放逻辑
		const dispose = this.disposeResources.bind(
			this,
			scene,
			renderer,
			stopRenderLoop
		);

		startRenderLoop(); // 启动渲染循环

		// 返回核心元素和方法
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

	/**
	 * 创建场景
	 */
	private static createScene(): THREE.Scene {
		return new THREE.Scene();
	}

	/**
	 * 创建透视相机
	 */
	private static createPerspectiveCamera(
		fov: number,
		aspect: number,
		near: number,
		far: number,
		position: THREE.Vector3
	): THREE.PerspectiveCamera {
		const camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
		camera.position.copy(position);
		return camera;
	}

	/**
	 * 创建渲染器
	 */
	private static createRenderer(
		width: number,
		height: number,
		options: THREE.WebGLRendererParameters
	): THREE.WebGLRenderer {
		const renderer = new THREE.WebGLRenderer(options);
		renderer.setSize(width, height);
		return renderer;
	}

	/**
	 * 创建 FlyControls
	 */
	private static createFlyControls(
		camera: THREE.PerspectiveCamera,
		domElement: HTMLElement,
		movementSpeed: number,
		rollSpeed: number,
		dragToLook: boolean
	): FlyControls {
		const controls = new FlyControls(camera, domElement);
		controls.movementSpeed = movementSpeed;
		controls.rollSpeed = rollSpeed;
		controls.dragToLook = dragToLook;
		return controls;
	}

	/**
	 * 检查点是否在地图范围内
	 */
	private static isInsideMap(
		camera: THREE.PerspectiveCamera,
		config: {
			rowNum: number;
			columnNum: number;
			exceeding: number;
		}
	): boolean {
		return (
			camera.position.x >= 0 &&
			camera.position.x <= this.halfSqrt3 * config.columnNum &&
			camera.position.y >= camera.position.x / this.sqrt3 &&
			camera.position.y <=
				config.rowNum + camera.position.x / this.sqrt3 &&
			camera.position.z >= config.exceeding
		);
	}

	/**
	 * 处理相机的边界逻辑
	 */
	private static wrapCameraPosition(
		camera: THREE.PerspectiveCamera,
		config: {
			rowNum: number;
			columnNum: number;
			exceeding: number;
		}
	): void {
		const makeUp = config.exceeding * this.epsilon;

		// 避免 z 轴超出
		if (camera.position.z < config.exceeding) {
			camera.position.z = config.exceeding;
		}

		// 环绕处理
		if (camera.position.x > this.halfSqrt3 * config.columnNum) {
			camera.position.x -= this.halfSqrt3 * config.columnNum - makeUp;
			camera.position.y -= 0.5 * config.columnNum - makeUp;
		} else if (camera.position.x < 0) {
			camera.position.x += this.halfSqrt3 * config.columnNum - makeUp;
			camera.position.y += 0.5 * config.columnNum - makeUp;
		}

		if (
			camera.position.y >
			config.rowNum + camera.position.x / this.sqrt3
		) {
			camera.position.y -= config.rowNum - makeUp;
		} else if (camera.position.y < camera.position.x / this.sqrt3) {
			camera.position.y += config.rowNum - makeUp;
		}
	}

	/**
	 * 启动渲染循环
	 */
	private static startRenderLoop(
		scene: THREE.Scene,
		camera: THREE.PerspectiveCamera,
		renderer: THREE.WebGLRenderer,
		controls: FlyControls,
		mapDimensions: { rowNum: number; columnNum: number },
		checkInterval: number,
		frameTime: number = 0.02
	): void {
		this.renderLoopActive = true;
		let frameCount = 0;

		const exceeding = controls.movementSpeed * checkInterval * frameTime;

		const config = {
			rowNum: mapDimensions.rowNum,
			columnNum: mapDimensions.columnNum,
			exceeding: exceeding,
		};

		const loop = () => {
			if (!this.renderLoopActive) return;

			requestAnimationFrame(loop);

			// 边界检查逻辑
			frameCount++;
			if (frameCount % checkInterval === 0) {
				frameCount = 0;

				if (!this.isInsideMap(camera, config)) {
					this.wrapCameraPosition(camera, config);
				}
			}

			// 更新控制器并渲染场景
			controls.update(frameTime);
			renderer.render(scene, camera);
		};
		loop();
	}

	/**
	 * 停止渲染循环
	 */
	private static stopRenderLoop() {
		this.renderLoopActive = false;
	}

	/**
	 * 清理资源
	 */
	private static disposeResources(
		scene: THREE.Scene,
		renderer: THREE.WebGLRenderer,
		stopRenderLoop: () => void
	) {
		stopRenderLoop();
		scene.children.forEach((child) => {
			if (child instanceof THREE.Mesh) {
				child.geometry.dispose();
				if (child.material instanceof THREE.Material) {
					child.material.dispose();
				}
			}
			scene.remove(child);
		});
		renderer.domElement.remove();
		renderer.dispose();
	}
}
