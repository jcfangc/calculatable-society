import { Ref } from "vue";
import {
	ThreeCoreFactory,
	ThreeCore,
	ThreeCoreOptions,
} from "./threeCoreFactory";
import { HotkeysManager } from "./hotkeysManager";
import { HexagonGridManager, HexagonGridConfig } from "./hexagonGridManager";
import { DynamicAdjustmentManager } from "./dynamicAdjustmentManager";

export class ThreeManager {
	private core!: ThreeCore; // 核心元素集成对象
	private hotkeysManager!: HotkeysManager; // 热键管理器
	private dynamicAdjustmentManager!: DynamicAdjustmentManager; // 动态调整管理器
	private options: ThreeCoreOptions; // 核心配置

	constructor(
		options: ThreeCoreOptions,
		isActive: Ref<boolean>,
		gridConfig: Omit<HexagonGridConfig, "scene" | "camera"> // 不包含场景和相机，由 ThreeCore 提供
	) {
		this.options = options;

		// 使用 ThreeCoreFactory 创建核心元素
		this.core = ThreeCoreFactory.createCore(this.options);

		// 初始化动态调整管理器
		this.dynamicAdjustmentManager = new DynamicAdjustmentManager(
			this.core.renderer,
			this.core.camera,
			this.options.width,
			this.options.height
		);

		// 创建完整的 HexagonGridConfig
		const fullGridConfig: HexagonGridConfig = {
			...gridConfig,
			scene: this.core.scene,
			camera: this.core.camera,
		};

		// 创建六边形网格并添加到场景
		const hexagonMatrix =
			HexagonGridManager.createHexagonGrid(fullGridConfig);

		// 自动计算相机默认位置
		const { defaultPosition, defaultLookAt } =
			HexagonGridManager.calculateDefaultCameraPosition(
				fullGridConfig,
				hexagonMatrix
			);

		// 初始化热键管理器并绑定默认相机位置
		this.hotkeysManager = new HotkeysManager(this.core.camera, isActive);
		this.hotkeysManager.setDefaultCameraPosition(
			defaultPosition,
			defaultLookAt
		);
	}

	/**
	 * 清理资源
	 */
	dispose() {
		// 清理动态调整管理器
		this.dynamicAdjustmentManager.dispose();

		// 调用核心 dispose 方法
		this.core.dispose();

		// 清理热键管理器
		this.hotkeysManager.dispose();
	}

	/**
	 * 获取核心元素
	 */
	getCore() {
		return this.core;
	}
}
