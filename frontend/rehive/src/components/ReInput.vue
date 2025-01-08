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
		@input.stop="updateInputValue"
	/>
</template>

<script lang="ts" setup>
	// 导入必要依赖和类型
	import { ref } from "vue";
	import { useCancelableDebounce } from "@/hooks/useCancelableDebounce.hook";
	import type { UseCancelableDebounceOptions } from "@/hooks/useCancelableDebounce.hook";

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
		onInputCallback: {
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

	// 更新输入框的值
	const updateInputValue = (event: Event) => {
		const inputElement = event.target as HTMLInputElement;
		const newValue = inputElement.value;
		// 使用原生验证
		validateInput(inputElement);
		// 即时更新父组件的值（支持双向绑定）
		emit("update:inputValue", newValue); // 更新值
	};

	// 使用自定义钩子函数
	const { watchValue } = useCancelableDebounce<string>(
		props.onInputCallback,
		{
			initialValue: props.inputValue,
			debounceDelay: 1000,
		} as UseCancelableDebounceOptions<string>
	);

	watchValue(() => props.inputValue);
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
