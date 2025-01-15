<template>
	<div :class="`tip tip--${level}`">
		<span class="tip__icon">{{ icon }}</span>
		{{ message }}
	</div>
</template>
<script lang="ts">
	import { defineComponent, computed, ref } from "vue";

	export const enum TipLevel {
		Success = "success",
		Warning = "warning",
		Error = "error",
		Info = "info",
	}

	export default defineComponent({
		name: "Tip",
		props: {
			level: {
				type: String as () => TipLevel,
				required: true,
			},
			message: {
				type: String,
				required: true,
			},
		},
		setup(props) {
			const icon = computed(() => {
				switch (props.level) {
					case TipLevel.Success:
						return "✅";
					case TipLevel.Warning:
						return "☢️";
					case TipLevel.Error:
						return "🚫";
					case TipLevel.Info:
					default:
						return "ℹ️";
				}
			});

			return {
				icon,
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
			$border-color: none
		);
		@include layout-config(
			$padding-y: var(--spacing-sm),
			$padding-x: var(--spacing-md),
			$gap: var(--spacing-sm),
			$max-width: 50vw,
			$position: fixed
		);
		z-index: 9999;
		top: 10%;
		left: 50%;
		transform: translate(-50%, -50%);
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
			$border-color: transparent
		);
		box-shadow: 0 0 1rem var(--success-border);
	}

	.tip--warning {
		@include colour-config(
			$text-color: white,
			$bg-color: var(--warning-bg),
			$border-color: transparent
		);
		box-shadow: 0 0 1rem var(--warning-border);
	}

	.tip--error {
		@include colour-config(
			$text-color: white,
			$bg-color: var(--error-bg),
			$border-color: transparent
		);
		box-shadow: 0 0 1rem var(--error-border);
	}

	.tip--info {
		@include colour-config(
			$text-color: white,
			$bg-color: var(--info-bg),
			$border-color: transparent
		);
		box-shadow: 0 0 1rem var(--info-border);
	}
</style>
