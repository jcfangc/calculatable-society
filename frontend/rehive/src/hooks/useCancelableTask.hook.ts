/**
 * 定义可选参数类型
 * - beforeStart: 开始时的钩子
 * - onComplete: 异步任务完成后的钩子
 * - onAbort: 异步任务被取消时的钩子
 * - taskLogic: 真正需要执行的异步逻辑
 */
export interface CancellableTaskOptions<T> {
	beforeStart?: (value: T) => void; // 开始时的钩子
	onComplete?: () => void; // 异步任务完成后的钩子
	onAbort?: () => void; // 异步任务被取消时的钩子
	taskLogic: (value: T, signal?: AbortSignal) => Promise<void>; // 具体的任务逻辑
}

/**
 * 提供一个「可取消的异步任务」函数
 * - 内部包含取消功能，对 `abortSignal` 的监听
 * - 允许用户自定义生命周期钩子和任务逻辑
 * @param options 自定义任务配置，比如日志输出、任务逻辑等
 * @returns 一个「可取消异步任务」函数 (value, signal?) => Promise<void>
 */
export function useCancellableTask<T>(options: CancellableTaskOptions<T>) {
	const { beforeStart, onComplete, onAbort, taskLogic } = options;

	if (typeof taskLogic !== "function") {
		throw new Error("taskLogic 必须定义，请提供具体的任务逻辑。");
	}

	/**
	 * 返回一个可取消的异步任务函数
	 * @param value 指定类型的值（通过泛型 T）
	 * @param abortSignal 用于取消任务
	 */
	return async function taskFunction(
		value: T,
		abortSignal?: AbortSignal
	): Promise<void> {
		// 开始执行前，可以调用 beforeStart 钩子
		if (typeof beforeStart === "function") {
			beforeStart(value);
		}

		try {
			// 执行用户定义的任务逻辑
			await taskLogic(value, abortSignal);

			// 任务完成后调用 onComplete 钩子
			if (typeof onComplete === "function") {
				onComplete();
			}
		} catch (error) {
			// 如果任务被取消，触发 onAbort 钩子
			if (abortSignal?.aborted && typeof onAbort === "function") {
				onAbort();
			}
			// 抛出其他错误，供外部捕获
			throw error;
		}
	};
}
