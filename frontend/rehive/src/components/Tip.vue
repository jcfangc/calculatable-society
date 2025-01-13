<template>
	<div
		v-if="visible"
		:class="`tip tip--${tip.level}`"
	>
		<span class="tip__icon">{{ icon }}</span>
		{{ tip.message }}
	</div>
</template>

<script lang="ts">
	import {
		defineComponent,
		reactive,
		onMounted,
		onUnmounted,
		computed,
		ref,
	} from "vue";
	import { emitter } from "@/utils/mitt/emitter";
	import {
		TipEvent,
		TipLevel,
		TipMessage,
	} from "@/utils/mitt/events/tipEvent";

	export default defineComponent({
		name: "Tip",
		setup() {
			// 响应式数据
			const tip = reactive<TipMessage>({
				level: TipLevel.Info,
				message: "",
			});

			const visible = ref(false);

			// 动态计算前缀符号
			const icon = computed(() => {
				switch (tip.level) {
					case TipLevel.Success:
						return "✅"; // 成功
					case TipLevel.Warning:
						return "☢️"; // 警告
					case TipLevel.Error:
						return "🚫"; // 错误
					case TipLevel.Info:
					default:
						return "ℹ️"; // 信息
				}
			});

			// 监听 Tip 事件
			const onShowTip = (message: TipMessage) => {
				tip.level = message.level;
				tip.message = message.message;
				visible.value = true;

				// 自动隐藏提示
				setTimeout(() => {
					visible.value = false;
				}, 3000);
			};

			onMounted(() => {
				emitter.on(TipEvent.Show, onShowTip);
			});

			onUnmounted(() => {
				emitter.off(TipEvent.Show, onShowTip);
			});

			return {
				tip,
				icon,
				visible,
			};
		},
	});
</script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	/* 基本样式 */
	.tip {
		@include default-config($colour-config: false, $effects-config: false);
		@include colour-config(
			$text-color: white,
			$bg-color: var(--info-bg),
			$border-color: var(--info-border)
		);
		@include layout-config(
			$padding-y: var(--spacing-md),
			$padding-x: var(--spacing-lg),
			$gap: var(--spacing-sm),
			$max-width: 50vw
		);
	}

	.tip__icon {
		@include default-config(
			$colour-config: false,
			$effects-config: false,
			$layout-config: false
		);
		@include colour-config(
			$text-color: white,
			$bg-color: white,
			$border-color: transparent
		);
		@include layout-config(
			$padding-y: var(--spacing-xs),
			$padding-x: var(--spacing-xs),
			$border-style: none
		);
	}

	/* 不同状态的样式 */
	.tip--success {
		@include colour-config(
			$text-color: white,
			$bg-color: var(--success-bg),
			$border-color: var(--success-border)
		);
	}

	.tip--warning {
		@include colour-config(
			$text-color: white,
			$bg-color: var(--warning-bg),
			$border-color: var(--warning-border)
		);
	}

	.tip--error {
		@include colour-config(
			$text-color: white,
			$bg-color: var(--error-bg),
			$border-color: var(--error-border)
		);
	}

	.tip--info {
		@include colour-config(
			$text-color: white,
			$bg-color: var(--info-bg),
			$border-color: var(--info-border)
		);
	}
</style>
