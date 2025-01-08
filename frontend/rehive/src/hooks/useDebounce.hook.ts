// hooks/useDebounce.ts
import { ref } from "vue";

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
