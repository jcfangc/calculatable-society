import hotkeys from "hotkeys-js";
import { Ref } from "vue";
import { TipLevel } from "@/components/Tip.vue";
import { useTip } from "@/hooks/useTip";
import * as THREE from "three";

export function registerReverseCameraHotkey(
	camera: THREE.PerspectiveCamera,
	isActive: Ref<boolean>
) {
	hotkeys("alt+r", () => {
		if (isActive.value) {
			camera.rotation.y += Math.PI;
			useTip(
				TipLevel.Success,
				`Reversed camera by 180 degrees. Now: x=${THREE.MathUtils.radToDeg(
					camera.rotation.x
				).toFixed(3)}°, y=${THREE.MathUtils.radToDeg(
					camera.rotation.y
				).toFixed(3)}°, z=${THREE.MathUtils.radToDeg(
					camera.rotation.z
				).toFixed(3)}°`
			);
			console.log("Camera reversed 180 degrees.");
		}
	});
}
