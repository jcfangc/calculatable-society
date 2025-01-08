<template>
	<button
		class="btn"
		:type="type"
		:disabled="disabled"
		:aria-disabled="disabled"
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
	import { defineProps, defineEmits, ref } from "vue";
	import { ButtonType, ButtonEvent } from "@/enums/button.enum";
	import { useDebounce } from "@/hooks/useDebounce.hook";
	import { useCancel } from "@/hooks/useCancel.hook";

	const props = defineProps({
		type: {
			type: String as () => ButtonType,
			default: ButtonType.Button,
		},
		disabled: {
			type: Boolean,
			default: false,
		},
		ariaLabel: {
			type: String,
			default:
				"这是一个按钮，可能是开发人员太忙了，还没为它设置描述，下次再来探索吧！",
		},
		// 点击回调函数，返回 Promise<void> 的异步函数
		onClickCallback: {
			type: Function as unknown as () => (
				event: MouseEvent
			) => Promise<void>,
			required: true,
		},
	});

	// 触发的自定义事件
	const emit = defineEmits([ButtonEvent.Click]);
	const buttonLoading = ref(false);
	const { debounce } = useDebounce();
	const { cancelCurrentRequest } = useCancel();

	async function handleClick(event: MouseEvent) {
		// 如果按钮已禁用或正在 loading，就不处理后续点击
		if (props.disabled) return;
		// 每次值变更时，先取消上一次的请求
		cancelCurrentRequest();
		// 再通过 debounce 延迟执行
		debounce(() => {
			buttonLoading.value = true;
			props
				.onClickCallback(event)
				.then(() => {
					emit(ButtonEvent.Click, event);
				})
				.finally(() => {
					buttonLoading.value = false;
				});
		}, 500);
	}
</script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.btn {
		@include default-config;
		cursor: pointer; // 光标样式
	}
</style>
