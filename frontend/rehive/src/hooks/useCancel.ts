import { ref, onUnmounted } from "vue";

/**
 * 专门处理请求取消逻辑的 Hook
 *
 * @returns 返回一个对象，包含以下属性和方法：
 *
 * Properties:
 * - loading: Ref<boolean>
 *   表示当前是否存在正在进行的异步请求。
 *   **适用场景**：可以用于显示加载动画或禁用输入框、按钮等交互操作。
 *   **示例用法**：`v-if="loading"` 或 `:disabled="loading"`。
 *
 * Methods:
 * - getSignal(): AbortSignal
 *   获取当前请求的 AbortSignal，用于传递给外部的异步请求函数，以支持取消操作。
 *   **适用场景**：当外部请求函数需要支持取消时，将此信号传递给请求对象，如 `fetch(url, { signal })`。
 *   **示例用法**：`const signal = getSignal(); fetchData(params, signal);`
 *
 * - startRequest(requestFn: () => Promise<void>): Promise<void>
 *   开始执行异步请求，同时维护 loading 状态。如果请求被取消或发生错误，会自动捕获和处理异常。
 *   **参数**：
 *   - `requestFn`：一个返回 Promise 的异步函数，函数内部需要支持 AbortSignal。
 *   **适用场景**：当需要执行异步请求时调用此方法，并将具体的请求逻辑传递给 `requestFn`。
 *   **示例用法**：
 *   ```ts
 *   startRequest(async () => {
 *     const response = await fetchData(params, getSignal());
 *     console.log(response);
 *   });
 *   ```
 *
 * - cancelCurrentRequest(): void
 *   取消当前正在执行或即将执行的请求，同时重置 loading 状态。
 *   **适用场景**：在用户输入变化过快或切换页面时调用，避免不必要的请求浪费资源。
 *   **示例用法**：
 *   ```ts
 *   watch(inputValue, () => {
 *     cancelCurrentRequest(); // 取消上一个请求
 *     startRequest(fetchData);
 *   });
 *   ```
 *
 * Cleanup:
 * - 在组件卸载时，自动取消未完成的请求，防止内存泄漏或异常行为。
 *   不需要手动调用，生命周期会自动触发。
 *
 * Usage Example:
 * ```ts
 * const {
 *   loading,
 *   getSignal,
 *   startRequest,
 *   cancelCurrentRequest
 * } = useCancel();
 *
 * async function fetchData() {
 *   await startRequest(async () => {
 *     const response = await fetch("/api/data", { signal: getSignal() });
 *     console.log(response);
 *   });
 * }
 *
 * // 取消请求
 * cancelCurrentRequest();
 * ```
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
		/**
		 * 当前是否存在正在进行的异步请求
		 * **适用场景**：可以用于显示加载动画或禁用输入框、按钮等交互操作。
		 * **示例用法**：`v-if="loading"` 或 `:disabled="loading"`。
		 */
		loading,

		/**
		 * 获取当前请求的 AbortSignal，用于传递给外部的异步请求函数，以支持取消操作。
		 * **适用场景**：当外部请求函数需要支持取消时，将此信号传递给请求对象，如 `fetch(url, { signal })`。
		 * **示例用法**：`const signal = getSignal(); fetchData(params, signal);`
		 */
		getSignal,

		/**
		 * 开始执行异步请求，同时维护 loading 状态。如果请求被取消或发生错误，会自动捕获和处理异常。
		 * **参数**：
		 * - `requestFn`：一个返回 Promise 的异步函数，函数内部需要支持 AbortSignal。
		 * **适用场景**：当需要执行异步请求时调用此方法，并将具体的请求逻辑传递给 `requestFn`。
		 */
		startRequest,

		/**
		 * 取消当前正在执行或即将执行的请求，同时重置 loading 状态。
		 * **适用场景**：在用户输入变化过快或切换页面时调用，避免不必要的请求浪费资源。
		 */
		cancelCurrentRequest,
	};
}
