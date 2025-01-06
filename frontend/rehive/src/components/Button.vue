<template>
	<button
		class="btn"
		:type="type"
		:disabled="disabled || isLoading"
		:aria-pressed="isLoading"
		v-bind="$attrs"
		@click.stop="$emit(ButtonEvent.Click)"
	>
		<!-- 使用作用域插槽 -->
		<slot>
			<span>按钮</span>
		</slot>
	</button>
</template>

<script lang="ts" setup>
	// 导入全局枚举
	import { ButtonType } from "@/enums/button.enum";
	import { ButtonEvent } from "@/enums/button.enum";

	defineProps({
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
		// 加载状态
		isLoading: {
			type: Boolean,
			default: false,
		},
	});
</script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.btn {
		// 布局配置
		@include layout-config();

		// 排版配置
		@include typography-config();

		// 颜色配置
		@include colour-config();

		// 效果配置
		@include effects-config();

		cursor: pointer; // 光标样式
	}
</style>
