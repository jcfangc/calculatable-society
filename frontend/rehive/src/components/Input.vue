<template>
	<input
		class="input"
		v-bind="$attrs"
	/>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.input {
		@include layout-config();
		@include typography-config();
		@include colour-config();
		@include effects-config();
	}
</style>
