import * as THREE from "three";

/**
 * 动态调整管理器
 */
export class DynamicAdjustmentManager {
	private renderer: THREE.WebGLRenderer;
	private camera: THREE.PerspectiveCamera;
	private width: number;
	private height: number;
	private handleResizeBound: () => void;

	constructor(
		renderer: THREE.WebGLRenderer,
		camera: THREE.PerspectiveCamera,
		width: number,
		height: number
	) {
		this.renderer = renderer;
		this.camera = camera;
		this.width = width;
		this.height = height;

		// 绑定调整事件的逻辑
		this.handleResizeBound = this.handleResize.bind(this);

		this.initListeners();
	}

	/**
	 * 初始化监听器
	 */
	private initListeners() {
		// 添加窗口调整事件监听
		window.addEventListener("resize", this.handleResizeBound);

		// 初始化时执行一次调整逻辑
		this.handleResize();
	}

	/**
	 * 动态调整窗口大小
	 */
	private handleResize() {
		// 更新相机和渲染器尺寸
		this.camera.aspect = this.width / this.height;
		this.camera.updateProjectionMatrix();
		this.renderer.setSize(this.width, this.height);
	}

	/**
	 * 清理所有动态调整的监听器
	 */
	dispose() {
		// 移除窗口调整事件监听
		window.removeEventListener("resize", this.handleResizeBound);
	}
}
