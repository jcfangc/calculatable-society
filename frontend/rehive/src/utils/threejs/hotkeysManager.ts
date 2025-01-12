import * as THREE from "three";
import hotkeys from "hotkeys-js";
import { Ref } from "vue";
import { useHexagonStore } from "@/stores/hexagonStore";

/**
 * 管理快捷键相关的类
 */
export class HotkeysManager {
	private camera: THREE.PerspectiveCamera;

	// 默认位置和朝向
	private defaultPosition: THREE.Vector3 = new THREE.Vector3(0, 0, 5);
	private defaultTarget: THREE.Vector3 = new THREE.Vector3(0, 0, 0);

	private zValueInput: string = "";
	private coordinateInput: string = "";

	// 控制快捷键是否启用
	private isActive: Ref<boolean>;

	constructor(camera: THREE.PerspectiveCamera, isActive: Ref<boolean>) {
		this.camera = camera;
		this.isActive = isActive;

		this.initHotkeys();
	}

	/**
	 * 初始化快捷键
	 */
	private initHotkeys() {
		// Alt + G: 重置相机位置
		hotkeys("alt+g", () => {
			if (this.isActive.value) this.resetCameraPosition();
		});

		// Alt + R: 摄像头反转 180 度
		hotkeys("alt+r", () => {
			if (this.isActive.value) this.reverseCamera();
		});

		// Z + 数字 + Enter: 更新相机 Z 坐标
		hotkeys("z+*", (event) => {
			if (this.isActive.value && !isNaN(Number(event.key))) {
				this.zValueInput += event.key;
			}
		});

		// G + 数字 + 中英文逗号 + 数字 + Enter: 移动到指定坐标
		hotkeys("g+*", (event) => {
			if (this.isActive.value) {
				// 允许数字和逗号输入
				if (
					!isNaN(Number(event.key)) ||
					event.key === "," ||
					event.key === "，"
				) {
					this.coordinateInput += event.key;
				}
			}
		});

		// Enter: 处理输入内容
		hotkeys("enter", () => {
			if (this.isActive.value) {
				if (this.zValueInput) {
					const zValue = parseFloat(this.zValueInput);
					this.updateCameraZ(zValue);
					this.zValueInput = "";
				}
				if (this.coordinateInput) {
					this.updateCameraPositionFromInput(this.coordinateInput);
					this.coordinateInput = "";
				}
			}
		});
	}

	/**
	 * 重置相机到默认位置
	 */
	private resetCameraPosition() {
		this.camera.position.copy(this.defaultPosition);
		this.camera.lookAt(this.defaultTarget);
		console.log(
			`Camera reset to default position: ${this.defaultPosition.toArray()}, target: ${this.defaultTarget.toArray()}`
		);
	}

	/**
	 * 设置相机的默认位置和目标
	 * @param position 默认位置
	 * @param target 默认目标
	 */
	public setDefaultCameraPosition(
		position: THREE.Vector3,
		target: THREE.Vector3
	) {
		this.defaultPosition.copy(position);
		this.defaultTarget.copy(target);
		console.log(
			`Default camera position set to: ${position.toArray()}, target: ${target.toArray()}`
		);
	}

	/**
	 * 摄像头反转 180 度
	 */
	private reverseCamera() {
		this.camera.rotation.y += Math.PI;
		console.log("Camera reversed 180 degrees.");
	}

	/**
	 * 更新摄像机的 Z 坐标
	 * @param zValue 用户输入的 Z 坐标值
	 */
	private updateCameraZ(zValue: number) {
		this.camera.position.z = zValue;
		console.log(`Camera Z position updated to: ${zValue}`);
	}

	/**
	 * 根据用户输入更新相机位置
	 * @param input 用户输入的坐标字符串
	 */
	private updateCameraPositionFromInput(input: string) {
		// 替换中英文逗号
		const sanitizedInput = input.replace(/，/g, ",");
		const [xStr, yStr] = sanitizedInput.split(",");

		if (xStr && yStr) {
			const x = parseFloat(xStr);
			const y = parseFloat(yStr);

			const { calculateHexagonCenter } = useHexagonStore();
			const { x: newX, y: newY } = calculateHexagonCenter(y, x);

			this.camera.position.set(newX, newY, 10);

			const toLookAt = new THREE.Vector3(newX, newY, 0);
			this.camera.lookAt(toLookAt);
			console.log(`Camera moved to position: [${newX}, ${newY}, 10]`);
		} else {
			console.error(
				"Invalid coordinate input. Expected format: 'number,number'."
			);
		}
	}

	/**
	 * 取消所有热键绑定
	 */
	public dispose() {
		hotkeys.unbind("alt+g");
		hotkeys.unbind("alt+r");
		hotkeys.unbind("z+*");
		hotkeys.unbind("g+*");
		hotkeys.unbind("enter");
		console.log("Hotkeys unbound.");
	}
}
