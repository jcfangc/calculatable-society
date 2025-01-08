<template>
	<Listbox>
		<!-- 按钮展示当前选中的选项 -->
		<ListboxButton class="btn">
			{{ selectedOption || "请选择选项" }}
		</ListboxButton>

		<!-- 下拉框选项 -->
		<ListboxOptions class="option-container">
			<ListboxOption
				v-for="option in options"
				:key="options.indexOf(option)"
				class="option"
				@click.stop="updateSelectValue(option)"
			>
				{{ option }}
			</ListboxOption>
		</ListboxOptions>
	</Listbox>
</template>

<script setup lang="ts">
	import {
		Listbox,
		ListboxButton,
		ListboxOptions,
		ListboxOption,
	} from "@headlessui/vue";
	import { useCancelableDebounce } from "@/hooks/useCancelableDebounce.hook";
	import type { UseCancelableDebounceOptions } from "@/hooks/useCancelableDebounce.hook";

	// 接收父组件传递的属性
	const props = defineProps({
		options: {
			type: Array as () => (string | number)[],
			required: true,
		},
		selectedOption: {
			type: String as () => string | number,
			default: "请选择选项",
		},
		// 回调函数：选项变更时触发
		onSelectCallback: {
			type: Function as unknown as () => (
				value: string | number,
				abort?: AbortSignal
			) => Promise<void>,
			required: true,
		},
	});

	const { watchValue } = useCancelableDebounce<string | number>(
		props.onSelectCallback,
		{
			initialValue: props.selectedOption,
			debounceDelay: 1000,
		} as UseCancelableDebounceOptions<string | number>
	);

	// 定义事件，用于更新 v-model
	const emit = defineEmits(["update:selectedOption"]);

	const updateSelectValue = (value: string | number) => {
		// 更新选中的值
		emit("update:selectedOption", value);
	};

	watchValue(() => props.selectedOption);
</script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.listbox {
		flex-direction: column;
	}

	.btn {
		@include default-config;
		cursor: pointer;
	}

	.option-container {
		@include default-config($layout-config: false, $effects-config: false);
		@include layout-config(
			$padding-x: var(--spacing-sm),
			$flex-direction: column,
			$justify-content: flex-start,
			$gap: var(--spacing-sm),
			$overflow: auto
		);
		@include effects-config(
			$active-transform: none,
			$hover-border-color: transparent
		);
		@include scrollbar-config;

		/* CSS 运行时计算 */
		max-height: calc(var(--container-max-height) * 0.5);
	}

	.option {
		@include default-config;
		cursor: pointer;
	}
</style>
