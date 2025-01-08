import { ref, onUnmounted } from "vue";

/**
 * 专门处理请求取消逻辑的 Hook
 */
export function useCancel() {
	// 用于控制请求是否正在进行
	const loading = ref(false);

	// 通过 AbortController 来实现取消请求
	let abortController = new AbortController();

	/**
	 * 开始一个请求：在这里维护 loading 状态，执行请求函数，并处理错误或中止
	 * @param requestFn 外部传入的异步请求函数
	 */
	async function startRequest(requestFn: () => Promise<void>) {
		loading.value = true;
		try {
			await requestFn();
		} catch (error) {
			// 如果是 AbortError，则代表请求已被取消，不再处理后续
			if (error instanceof DOMException && error.name === "AbortError") {
				// do nothing
			} else {
				console.error("请求执行失败:", error);
			}
		} finally {
			loading.value = false;
		}
	}

	/**
	 * 取消当前请求
	 */
	function cancelCurrentRequest() {
		// 如果当前正在加载，则取消请求
		if (loading.value) {
			abortController.abort(); // 取消当前请求
			abortController = new AbortController(); // 重新创建一个新的 Controller
			loading.value = false;
		}
	}

	/**
	 * 返回当前信号，供外部作为 AbortSignal
	 */
	function getSignal() {
		return abortController.signal;
	}

	// 在组件卸载时，确保请求被取消，避免出现内存泄漏
	onUnmounted(() => {
		cancelCurrentRequest();
	});

	return {
		loading,
		getSignal,
		startRequest,
		cancelCurrentRequest,
	};
}
