// hooks/useDebounce.ts
import { ref } from "vue";

/**
 * 专门用于防抖的 Hook
 * @returns 返回一个对象，包含以下属性和方法：
 * - debounce(callback: () => void, delay?: number): void
 * 防抖函数，用于处理频繁触发的事件，只有在事件停止一段时间后才会执行回调。
 */
export function useDebounce() {
	const debounceTimer = ref<ReturnType<typeof setTimeout> | null>(null);

	/**
	 * 防抖函数
	 * @param callback 要执行的回调函数
	 * @param delay 防抖延迟时间，默认 1000ms
	 */
	const debounce = (callback: () => void, delay = 1000) => {
		// 清除已有的定时器
		if (debounceTimer.value) {
			clearTimeout(debounceTimer.value);
		}
		// 设置新的定时器
		debounceTimer.value = setTimeout(() => {
			callback(); // 执行回调
		}, delay);
	};

	return {
		debounce,
	};
}
