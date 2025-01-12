import { ref, watch } from "vue";
import type { Ref } from "vue";
import { useDebounce } from "@/hooks/useDebounce";
import { useCancel } from "@/hooks/useCancel";

/**
 * useCancelableDebounce 的配置项
 * @template T 泛型类型
 * @property {T} [initialValue] 初始值
 * @property {number} [debounceDelay] 防抖延迟
 * @property {boolean} [immediate] 是否在初始时立即执行
 * @property {boolean} [allowEmpty] 当值为空/undefined/null 时，是否依然执行回调
 */
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
 *
 * @param callback - 回调函数，接收参数值和 AbortSignal，用于执行异步操作。
 * @param options - 配置项，可控制初始值、防抖延迟、是否允许空值、是否立即执行等行为。
 *
 * @returns 返回一个对象，包含以下属性和方法：
 *
 * Properties:
 * - debouncedValue: Ref<T | undefined>
 *   防抖后的值，通过 ref 进行响应式管理，可以直接绑定到组件中使用。
 *
 * - loading: Ref<boolean>
 *   当前异步请求的加载状态，为 true 表示正在执行请求，可用于显示“加载中”动画或禁用输入框。
 *
 * Methods:
 * - cancelCurrentRequest(): void
 *   取消当前正在执行或即将执行的异步请求，同时重置 loading 状态。
 *   **适用场景**：用户快速切换输入或选项时，需要取消前一个未完成的请求。
 *
 * - startLoading(value: T): Promise<void>
 *   手动触发异步请求，适用于需要立即执行操作的场景。
 *   接收一个值作为参数，并调用回调函数处理该值。如果请求被取消或出错，会自动处理异常。
 *   **适用场景**：需要程序主动加载某个值时调用，而不是依赖防抖触发。
 *
 * - setValue(newVal: T): void
 *   设置防抖值 `debouncedValue`，支持手动更新值，但不会触发回调或异步请求。
 *   **适用场景**：需要在防抖之外手动设置内部状态值，比如表单重置或回显。
 *
 * - watchValue(externalValue: () => T): void
 *   监听外部传入值，当值变化时，会自动触发防抖逻辑并执行回调。
 *   **适用场景**：绑定组件的 props 或 v-model，实现双向绑定和自动更新。
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
		/**
		 * 防抖后的最新值，保持响应式，可供外部组件直接访问或绑定
		 * 示例：
		 * <input v-model="debouncedValue" />
		 */
		debouncedValue,

		/**
		 * 当前异步请求的加载状态，适用于显示 loading 动画或禁用按钮
		 * 示例：
		 * <button :disabled="loading">提交</button>
		 */
		loading,

		/**
		 * 取消当前正在进行的请求。
		 * - 当用户快速切换输入值或选项时，调用此函数取消前一个未完成的请求。
		 * 示例：
		 * cancelCurrentRequest();
		 */
		cancelCurrentRequest,

		/**
		 * 手动触发一次回调执行，适用于不依赖 watch 自动触发的场景。
		 * 参数：
		 * @param value - 要传递给回调函数的值
		 * 示例：
		 * startLoading("新值");
		 */
		startLoading,

		/**
		 * 设置防抖值，用于程序逻辑动态更新值，而非依赖用户输入
		 * 参数：
		 * @param newVal - 要设置的新值
		 * 示例：
		 * setValue("预设值");
		 */
		setValue,

		/**
		 * 监听外部值变化并自动执行防抖与回调。
		 * 参数：
		 * @param externalValue - 要监听的外部值
		 * 示例：
		 * watchValue(() => props.selectedOption);
		 */
		watchValue,
	};
}
