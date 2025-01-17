import { Ref } from "vue";
import * as THREE from "three";
import { FlyControls } from "three/examples/jsm/controls/FlyControls";
import {
	registerResetCameraHotkey,
	registerReverseCameraHotkey,
	registerUpdateCameraZHotkey,
	registerMoveCameraHotkey,
} from "@/utils/threejs/hotkeys";
import hotkeys from "hotkeys-js";

export class HotkeysManager {
	private camera: THREE.PerspectiveCamera;
	private isActive: Ref<boolean>;
	private controls: FlyControls;
	private defaultPosition: THREE.Vector3;
	private defaultTarget: THREE.Vector3;

	constructor(
		camera: THREE.PerspectiveCamera,
		isActive: Ref<boolean>,
		controls: FlyControls
	) {
		this.camera = camera;
		this.isActive = isActive;
		this.controls = controls;
		this.defaultPosition = new THREE.Vector3(0, 0, 5);
		this.defaultTarget = new THREE.Vector3(0, 0, 0);

		this.initHotkeys();
	}

	private initHotkeys() {
		registerResetCameraHotkey(
			this.camera,
			this.isActive,
			this.defaultPosition,
			this.defaultTarget
		);
		registerReverseCameraHotkey(this.camera, this.isActive);
		registerUpdateCameraZHotkey(
			this.camera,
			this.isActive,
			this.enableControls.bind(this)
		);
		registerMoveCameraHotkey(
			this.camera,
			this.isActive,
			this.enableControls.bind(this)
		);
	}

	private enableControls(enable: boolean) {
		this.isActive.value = enable;
		this.controls.enabled = enable;
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

	public dispose() {
		hotkeys.unbind("alt+g");
		hotkeys.unbind("alt+r");
		hotkeys.unbind("alt+z");
		hotkeys.unbind("alt+m");
		console.log("Hotkeys unbound.");
	}
}
