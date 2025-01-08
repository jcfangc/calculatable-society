<template>
	<input
		class="input"
		v-bind="$attrs"
		:value="inputValue"
		:disabled="disabled"
		:placeholder="placeholder"
		:aria-disabled="disabled"
		:aria-placeholder="placeholder"
		:aria-invalid="isInvalid"
		:aria-required="required"
		:required="required"
		:aria-label="ariaLabel || placeholder"
		@input="updateInputValue"
	/>
</template>

<script lang="ts" setup>
	// 导入必要依赖和类型
	import { ref, watch } from "vue";
	import { useDebounce } from "@/hooks/useDebounce.hook";

	// 定义 Props
	const props = defineProps({
		// 输入框的值
		inputValue: {
			type: String,
			required: true,
		},
		// 禁用状态
		disabled: {
			type: Boolean,
			default: false,
		},
		// 输入事件回调函数，允许默认参数为字符串内容
		inputCallback: {
			type: Function as unknown as () => (
				value: string,
				abortSignal?: AbortSignal
			) => Promise<void>,
			required: true,
		},
		// 占位符
		placeholder: {
			type: String,
			default: "请输入内容",
		},
		// ARIA
		// ARIA 属性：标签
		ariaLabel: {
			type: String,
			default:
				"这是一个输入框，可能是开发人员太忙了，还没为它设置描述，下次再来探索吧！",
		},
		// ARIA 属性：是否必填
		required: {
			type: Boolean,
			default: false,
		},
	});

	// 定义 Emits
	const emit = defineEmits([
		"update:inputValue", // 支持 v-model 的双向绑定
		// InputEvent.Input, // 自定义事件
	]);

	// 定义响应式变量
	const isInvalid = ref(false); // 输入是否合法
	// 使用原生 API 验证输入合法性
	const validateInput = (element: HTMLInputElement) => {
		if (!element.checkValidity()) {
			isInvalid.value = true; // 输入非法
		} else {
			isInvalid.value = false; // 输入合法
		}
	};

	const updateInputValue = (event: Event) => {
		const inputElement = event.target as HTMLInputElement;
		const newValue = inputElement.value;
		// 使用原生验证
		validateInput(inputElement);
		// 即时更新父组件的值（支持双向绑定）
		emit("update:inputValue", newValue); // 更新值
	};

	let inputLoading = false;
	const debouncedValue = ref(props.inputValue); // 防抖用的值
	let abortController = new AbortController(); // 使用 AbortController 取消请求
	const { debounce } = useDebounce(); // 使用自定义防抖 hook

	const cancelCurrentRequest = () => {
		// 如果正在加载，则立即中止当前请求
		if (inputLoading) {
			abortController.abort(); // 取消当前回调
			abortController = new AbortController(); // 重置取消控制器
			inputLoading = false; // 重置加载状态
		}
	};

	watch(
		() => props.inputValue,
		(newValue) => {
			// 取消当前请求
			cancelCurrentRequest();
			// 防抖处理并启动加载逻辑
			debounce(() => {
				if (newValue) {
					debouncedValue.value = newValue; // 更新防抖后的值
					startLoading(newValue); // 启动加载逻辑
				}
			}, 1000); // 防抖时间为 1 秒
		}
	);

	// 加载回调函数
	const startLoading = async (value: string) => {
		if (!value) return; // 如果输入值为空，则直接返回

		// 标记加载状态
		inputLoading = true;

		try {
			// 调用异步回调函数，并传递 abortSignal
			await props.inputCallback(value, abortController.signal);
		} catch (error) {
			// 错误处理
			if (error instanceof DOMException && error.name === "AbortError") {
			} else if (error instanceof Error) {
				console.error("回调执行失败:", error.message);
			} else {
				console.error("未知错误:", error);
			}
		} finally {
			// 恢复状态
			inputLoading = false;
		}
	};
</script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.input {
		@include default-config;
	}

	.input::placeholder {
		@include colour-config($bg-color: transparent);
	}
</style>
