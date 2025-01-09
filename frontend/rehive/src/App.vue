<template>
	<Sidebar />
</template>

<script setup lang="ts">
	import Sidebar from "@/components/Sidebar.vue";
</script>

<style lang="scss">
	// 引入全局样式
	@use "@/assets/style/default/_reset.scss" as *;

	// 应用容器样式
	#app {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 100vh; // 使用视口高度填充全屏
		background: var(--bg-primary);
	}
</style>
