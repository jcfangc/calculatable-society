<template>
	<!-- 自定义下拉框（仅列表实现） -->
	<ul
		class="spread"
		@click.stop="toggleSpread"
	>
		<!-- 当前选中项 -->
		<li
			class="spread-selected"
			v-if="!isOpen"
		>
			{{ selectedOption.label || "请选择" }}
		</li>

		<!-- 下拉选项 -->
		<li
			v-if="isOpen"
			v-for="item in options"
			:key="item.key"
			:class="['spread-item']"
			@click.stop="selectOption(item)"
		>
			{{ item.label }}
		</li>
	</ul>
</template>

<script lang="ts">
	import { defineComponent, ref } from "vue";

	// 定义 Option 类（内聚在组件内部，但对外暴露）
	export class Option {
		label: string;
		value: string | number;
		key: string | number;

		// 构造函数
		constructor(
			label: string,
			value: string | number,
			key: string | number
		) {
			this.label = label;
			this.value = value;
			this.key = key;
		}

		// 可选：定义辅助方法
		isEqual(option: Option): boolean {
			return (
				this.value === option.value &&
				this.label === option.label &&
				this.key === option.key
			);
		}
	}

	// 导出组件
	export default defineComponent({
		name: "Select",
		props: {
			options: {
				type: Array as () => Option[],
				required: true,
			},
		},
		emits: ["update:modelValue"],
		setup(_, { emit }) {
			// 状态
			const isOpen = ref(false);

			// 选中的选项
			const selectedOption = ref<Option>(new Option("", "", ""));

			// 切换下拉菜单显示状态
			const toggleSpread = () => {
				isOpen.value = !isOpen.value;
			};

			// 选择某个选项
			const selectOption = (option: Option) => {
				selectedOption.value = option;
				isOpen.value = false;
				emit("update:modelValue", option.value);
			};

			// 点击页面其他地方关闭下拉框
			const handleClickOutside = (event: MouseEvent) => {
				const target = event.target as HTMLElement;
				if (!target.closest(".spread")) {
					isOpen.value = false;
				}
			};

			document.addEventListener("click", handleClickOutside);

			// 返回模板绑定的属性和方法
			return {
				isOpen,
				selectedOption,
				toggleSpread,
				selectOption,
			};
		},
	});
</script>

<style scoped lang="scss">
	.spread {
		@include layout-config(
			$padding-x: var(--spacing-sm),
			$flex-direction: column,
			$justify-content: flex-start,
			$gap: var(--spacing-sm),
			$max-height: calc(var(--container-max-height) * 50%),
			$overflow: scroll,
			// 设置滚动条
		);
		@include typography-config();
		@include colour-config();
		@include effects-config();

		flex-grow: 0;
		cursor: pointer;

		&-selected {
			@include layout-config();
			@include typography-config();
			@include colour-config();
			@include effects-config();
		}

		&-item {
			@include layout-config();
			@include typography-config();
			@include colour-config();
			@include effects-config();
		}
	}
</style>
