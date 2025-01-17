import hotkeys from "hotkeys-js";
import { Ref } from "vue";
import { useTip } from "@/hooks/useTip";
import { TipLevel } from "@/components/Tip.vue";
import { useDynForm } from "@/hooks/useDynForm";
import { FormFieldType } from "@/components/DynForm.vue";
import * as THREE from "three";
import { useHexagonStore } from "@/stores/hexagonStore";

export function registerMoveCameraHotkey(
	camera: THREE.PerspectiveCamera,
	isActive: Ref<boolean>,
	enableControls: (enable: boolean) => void
) {
	hotkeys("alt+m", () => {
		if (isActive.value) {
			enableControls(false);
			useDynForm(
				[
					{
						name: "xValue",
						label: "请输入 X 坐标",
						type: FormFieldType.Number,
						placeholder: "例如：1",
					},
					{
						name: "yValue",
						label: "请输入 Y 坐标",
						type: FormFieldType.Number,
						placeholder: "例如：1",
					},
				],
				(formData) => {
					const xValue = parseFloat(formData.xValue);
					const yValue = parseFloat(formData.yValue);

					if (!isNaN(xValue) && !isNaN(yValue)) {
						const { calculateHexagonCenter } = useHexagonStore();
						const { x: newX, y: newY } = calculateHexagonCenter(
							yValue,
							xValue
						);

						camera.position.set(newX, newY, 10);
						camera.lookAt(new THREE.Vector3(newX, newY, 0));
						useTip(
							TipLevel.Success,
							// 小数点后保留 3 位
							`Camera Moved to: [${newX.toFixed(
								3
							)}, ${newY.toFixed(3)}]`
						);
					} else {
						useTip(TipLevel.Error, "无效的坐标输入！");
					}
					enableControls(true);
				},
				() => {
					useTip(TipLevel.Info, "取消了坐标移动。");
					enableControls(true);
				}
			);
		}
	});
}
