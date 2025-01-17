<template>
	<div class="form-overlay">
		<form
			class="user-form"
			@submit.prevent="handleSubmit"
		>
			<div
				class="form-field"
				v-for="field in fields"
				:key="field.name"
			>
				<label :for="field.name">{{ field.label }}</label>
				<input
					class="form-input"
					:id="field.name"
					:type="field.type"
					v-model="formData[field.name]"
					:placeholder="field.placeholder"
					ref="inputRefs"
				/>
			</div>
			<div class="form-actions">
				<button
					type="submit"
					class="form-submit"
				>
					提交
				</button>
				<button
					type="button"
					class="form-cancel"
					@click="handleCancel"
				>
					取消
				</button>
			</div>
		</form>
	</div>
</template>

<script lang="ts">
	import {
		defineComponent,
		reactive,
		ref,
		PropType,
		onMounted,
		onUnmounted,
	} from "vue";

	// 定义字段类型枚举
	export const enum FormFieldType {
		Text = "text",
		Password = "password",
		Email = "email",
		Number = "number",
	}

	// 定义字段接口
	export interface FormField {
		name: string;
		label: string;
		type: FormFieldType;
		placeholder?: string;
	}

	export default defineComponent({
		name: "DynForm",
		props: {
			fields: {
				type: Array as PropType<FormField[]>,
				required: true,
			},
			onSubmit: {
				type: Function as PropType<
					(formData: Record<string, string>) => void
				>,
				required: true,
			},
			onCancel: {
				type: Function as PropType<() => void>,
				required: false,
			},
		},
		setup(props) {
			// 动态生成的表单数据
			const formData = reactive(
				props.fields.reduce<Record<string, string>>((acc, field) => {
					acc[field.name] = "";
					return acc;
				}, {})
			);

			// 用于存储当前活动的焦点
			let previousActiveElement: HTMLElement | null = null;

			// 引用所有的输入框
			const inputRefs = ref<HTMLInputElement[]>([]);

			// 提交表单
			const handleSubmit = () => {
				props.onSubmit(formData);
			};

			// 取消表单
			const handleCancel = () => {
				props.onCancel?.();
			};

			onMounted(() => {
				// 保存当前活动的元素
				previousActiveElement =
					document.activeElement as HTMLElement | null;

				// 如果之前有焦点的元素，显式地让其失去焦点
				if (
					previousActiveElement &&
					typeof previousActiveElement.blur === "function"
				) {
					previousActiveElement.blur();
				}

				// 聚焦到第一个输入框
				if (inputRefs.value.length > 0) {
					inputRefs.value[0].focus();
				}
			});

			// 卸载时操作
			onUnmounted(() => {
				// 将焦点返回到之前的元素
				if (previousActiveElement) {
					previousActiveElement.focus();
				}
			});

			return {
				formData,
				handleSubmit,
				handleCancel,
				inputRefs,
			};
		},
	});
</script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.form-overlay {
		position: fixed;
		top: 0; /* 从页面顶部开始 */
		left: 0;
		width: 100%;
		height: 100%; /* 占满整个视口 */
		display: flex;
		justify-content: center; /* 水平居中 */
		align-items: center; /* 垂直居中 */
		background-color: rgba(0, 0, 0, 0.2); /* 添加背景遮罩 */
		// z-index: 9998;
	}

	.user-form {
		@include default-config($layout-config: false, $effects-config: false);
		@include layout-config(
			$gap: var(--spacing-sm),
			$flex-direction: column,
			$justify-content: flex-start,
			$max-height: 80vh,
			$overflow: auto,
			$align-items: flex-start
		);
		@include scrollbar-config;
	}

	.form-field {
		@include default-config($layout-config: false, $colour-config: false);
		@include layout-config(
			$gap: var(--spacing-xs),
			$flex-direction: column,
			$align-items: flex-start,
			$border-style: none
		);
		@include colour-config($bg-color: transparent);
	}

	.form-input {
		@include default-config;

		/* 正常状态的 placeholder 样式 */
		&::placeholder {
			@include colour-config($bg-color: transparent);
		}

		/* 输入框悬停时改变 placeholder 样式 */
		&:hover::placeholder {
			@include colour-config(
				$text-color: var(--bg-primary),
				$bg-color: transparent
			);
		}
	}

	.form-input[type="number"] {
		@include spin-button-config;
	}

	.form-actions {
		@include layout-config(
			$position: flex,
			$width: 100%,
			$flex-direction: column,
			$gap: var(--spacing-sm),
			$justify-content: flex-start,
			$border-style: none,
			$align-items: stretch // 子元素（按钮）撑满父容器
		);
	}

	.form-submit,
	.form-cancel {
		@include default-config($colour-config: flase);

		cursor: pointer;
	}

	.form-submit {
		@include default-config($colour-config: false);
		@include colour-config(
			$bg-color: var(--text-secondary),
			$text-color: var(--bg-primary),
			$border-color: var(--bg-primary)
		);
	}

	.form-cancel {
		@include default-config();
	}
</style>
