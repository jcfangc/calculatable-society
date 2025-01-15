import { createVNode, render } from "vue";
import Tip, { TipLevel } from "@/components/Tip.vue";

export function useTip(level: TipLevel, message: string): void {
	const container = document.createElement("div");
	document.body.appendChild(container);

	const vnode = createVNode(Tip, {
		level,
		message,
	});
	render(vnode, container);

	// 动态计算显示时间（每个字符 100ms，限制最短 2 秒，最长 10 秒）
	const displayTime = Math.min(Math.max(message.length * 100, 2000), 10000);

	setTimeout(() => {
		render(null, container);
		document.body.removeChild(container);
	}, displayTime);
}
