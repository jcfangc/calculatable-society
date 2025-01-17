import hotkeys from "hotkeys-js";
import { Ref } from "vue";
import { useTip } from "@/hooks/useTip";
import { TipLevel } from "@/components/Tip.vue";
import { useDynForm } from "@/hooks/useDynForm";
import { FormFieldType } from "@/components/DynForm.vue";
import * as THREE from "three";

export function registerUpdateCameraZHotkey(
	camera: THREE.PerspectiveCamera,
	isActive: Ref<boolean>,
	enableControls: (enable: boolean) => void
) {
	hotkeys("alt+z", () => {
		if (isActive.value) {
			enableControls(false);
			useDynForm(
				[
					{
						name: "zValue",
						label: "请输入新的 Z 坐标",
						type: FormFieldType.Number,
						placeholder: "例如：10",
					},
				],
				(formData) => {
					const zValue = parseFloat(formData.zValue);
					if (!isNaN(zValue) && zValue >= 0) {
						camera.position.z = zValue;
						useTip(TipLevel.Success, `Camera Z Updated: ${zValue}`);
					} else {
						useTip(TipLevel.Error, "无效的 Z 坐标输入！");
					}
					enableControls(true);
				},
				() => {
					useTip(TipLevel.Info, "取消了 Z 坐标更新。");
					enableControls(true);
				}
			);
		}
	});
}
