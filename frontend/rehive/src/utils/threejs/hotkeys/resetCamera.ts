import hotkeys from "hotkeys-js";
import { Ref } from "vue";
import { TipLevel } from "@/components/Tip.vue";
import { useTip } from "@/hooks/useTip";
import * as THREE from "three";

export function registerResetCameraHotkey(
	camera: THREE.PerspectiveCamera,
	isActive: Ref<boolean>,
	defaultPosition: THREE.Vector3,
	defaultTarget: THREE.Vector3
) {
	hotkeys("alt+g", () => {
		if (isActive.value) {
			camera.position.copy(defaultPosition);
			camera.lookAt(defaultTarget);
			useTip(
				TipLevel.Success,
				`Reset camera to default position: [${defaultPosition
					.toArray()
					.map((val) => val.toFixed(3))
					.join(", ")}]`
			);
			console.log("Camera reset to default position.");
		}
	});
}
