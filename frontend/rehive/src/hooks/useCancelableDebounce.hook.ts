import { ref, watch } from "vue";
import type { Ref } from "vue";
import { useDebounce } from "@/hooks/useDebounce.hook";
import { useCancel } from "@/hooks/useCancel.hook";

export interface UseCancelableDebounceOptions<T> {
	// 初始值
	initialValue?: T;
	// 防抖延迟
	debounceDelay?: number;
	// 是否在初始时立即执行
	immediate?: boolean;
	// 当值为空/undefined/null 时，是否依然执行回调
	allowEmpty?: boolean;
}

/**
 * 将“防抖 + 可取消请求”逻辑组合到一起的 Hook
 * @param callback 回调函数：接收 value 和 AbortSignal
 * @param options  配置项
 */
export function useCancelableDebounce<T>(
	callback: (value: T, signal: AbortSignal) => Promise<void>,
	options: UseCancelableDebounceOptions<T>
): {
	debouncedValue: Ref<T | undefined>;
	loading: Ref<boolean>;
	cancelCurrentRequest: () => void;
	startLoading: (value: T) => Promise<void>;
	setValue: (newVal: T) => void;
	watchValue: (externalValue: () => T) => void;
} {
	const {
		initialValue,
		debounceDelay = 1000,
		immediate = false,
		allowEmpty = false,
	} = options;

	// 将外部传入的值，先存到 ref 里
	const debouncedValue = ref<T | undefined>(initialValue) as Ref<
		T | undefined
	>;

	// === 使用我们拆分出来的 useCancel ===
	const { loading, getSignal, startRequest, cancelCurrentRequest } =
		useCancel();

	/**
	 * 真正执行回调的函数
	 */
	async function startLoading(value: T) {
		// 如果不允许空值 && 值为空，则直接返回
		if (
			!allowEmpty &&
			(value === null || value === "" || value === undefined)
		) {
			return;
		}
		// 调用 useCancel 提供的 startRequest，把要执行的异步逻辑在回调里传给它
		await startRequest(() => callback(value, getSignal()));
	}

	// 如果要在初始阶段立刻执行一次
	if (immediate && initialValue !== undefined) {
		startLoading(initialValue);
	}

	/**
	 * 提供一个方法：允许外部主动修改 debouncedValue
	 */
	function setValue(newVal: T) {
		debouncedValue.value = newVal;
	}

	/**
	 * 给外部用的 watch 逻辑，用来监听某个“外部值”并触发回调
	 * - 也可以不暴露 watchValue，而直接在组件中写 watch，然后手动调用 startLoading / cancelCurrentRequest
	 */
	function watchValue(externalValue: () => T) {
		const { debounce } = useDebounce();

		watch(
			externalValue,
			(newVal) => {
				// 每次值变更时，先取消上一次的请求
				cancelCurrentRequest();
				// 再通过 debounce 延迟执行
				debounce(() => {
					setValue(newVal);
					startLoading(newVal);
				}, debounceDelay);
			},
			{ immediate: false }
		);
	}

	return {
		debouncedValue,
		loading,
		cancelCurrentRequest,
		startLoading,
		setValue,
		watchValue,
	};
}
