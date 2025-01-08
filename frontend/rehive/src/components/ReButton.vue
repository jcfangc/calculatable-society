<template>
	<button
		class="btn"
		:type="type"
		:disabled="disabled || buttonLoading"
		:aria-disabled="disabled || buttonLoading"
		:aria-busy="buttonLoading"
		:aria-label="ariaLabel"
		:aria-pressed="buttonLoading"
		v-bind="$attrs"
		@click.stop="handleClick"
	>
		<!-- 使用作用域插槽 -->
		<slot>
			<span>按钮</span>
		</slot>
	</button>
</template>

<script lang="ts" setup>
	// 导入全局枚举
	import { ButtonType, ButtonEvent } from "@/enums/button.enum";
	import { ref } from "vue";

	// 定义 Props
	const props = defineProps({
		// 按钮类型
		type: {
			type: String as () => ButtonType,
			default: ButtonType.Button,
		},
		// 禁用状态
		disabled: {
			type: Boolean,
			default: false,
		},
		// 按钮的描述标签（用于屏幕阅读器）
		ariaLabel: {
			type: String,
			default:
				"这是一个按钮，可能是开发人员太忙了，还没为它设置描述，下次再来探索吧！",
		},
		// 点击回调函数，只允许传入异步函数
		onClickCallback: {
			// 限定必须返回 Promise<void> 的异步函数
			type: Function as unknown as () => (
				event: MouseEvent
			) => Promise<void>,
			required: true,
		},
	});

	// 定义事件
	const emit = defineEmits([ButtonEvent.Click]);
	// 定义响应式数据
	const buttonLoading = ref(false);

	const handleClick = async (event: MouseEvent) => {
		if (buttonLoading.value || props.disabled) return; // 如果正在加载中，则直接返回
		buttonLoading.value = true; // 设置加载中状态

		try {
			// 等待回调完成
			await props.onClickCallback(event);
			emit(ButtonEvent.Click, event); // 触发点击事件
		} catch (error) {
			console.error("回调执行失败:", error);
		}

		buttonLoading.value = false; // 取消加载中状态
	};
</script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.btn {
		@include default-config;
		cursor: pointer; // 光标样式
	}
</style>
