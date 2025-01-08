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
				@click="updateSelectValue(option)"
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
	import { ref, watch } from "vue";
	import { useDebounce } from "@/hooks/useDebounce.hook";

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
		selectCallback: {
			type: Function as unknown as () => (
				value: string | number,
				abort?: AbortSignal
			) => Promise<void>,
			required: true,
		},
	});

	const updateSelectValue = (value: string | number) => {
		// 更新选中的值
		emit("update:selectedOption", value);
	};

	let abortController = new AbortController(); // 使用 AbortController 取消请求
	let selectLoading = false;
	const debouncedValue = ref(props.selectedOption); // 防抖用的值
	const { debounce } = useDebounce(); // 使用自定义防抖 hook

	const cancelCurrentRequest = () => {
		// 如果正在加载，则立即中止当前请求
		if (selectLoading) {
			abortController.abort(); // 取消当前回调
			abortController = new AbortController(); // 重置取消控制器
			selectLoading = false; // 重置加载状态
		}
	};

	watch(
		() => props.selectedOption,
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

	// 定义事件，用于更新 v-model
	const emit = defineEmits(["update:selectedOption"]);

	// 监听用户选择更新 v-model
	// 用户选择后触发的逻辑
	const startLoading = async (value: string | number) => {
		selectLoading = true; // 设置加载状态

		// 如果存在回调函数，则执行回调
		try {
			await props.selectCallback(value, abortController.signal); // 调用父组件传入的回调
		} catch (error) {
			// 错误处理
			if (error instanceof DOMException && error.name === "AbortError") {
			} else if (error instanceof Error) {
				console.error("回调执行失败:", error.message);
			} else {
				console.error("未知错误:", error);
			}
		} finally {
			selectLoading = false; // 重置加载状态
		}
	};
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
