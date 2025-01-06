<template>
	<!-- 使用按钮组件 -->
	<Button></Button>
	<InputBox></InputBox>
	<Select :options="options"></Select>
</template>

<script setup lang="ts">
	import { ref } from "vue";
	import Button from "./components/Button.vue";
	import InputBox from "./components/Input.vue";
	import Select from "./components/Select.vue";
	import { Option } from "./components/Select.vue";

	// 选项数据
	const options = ref([
		new Option("选项A", "A", "1"),
		new Option("选项B", "B", "2"),
		new Option("选项C34", "C", "3"),
		new Option("选项CFD", "C", "4"),
		new Option("选项Cgggggggg", "C", "5"),
		new Option("选项Cgggggggg", "C", "6"),
		new Option("选项Cgggggggg", "C", "7"),
		new Option("选项Cgggggggg", "C", "8"),
		new Option("选项Cgggggggg", "C", "9"),
		new Option("选项Cgggggggg", "C", "10"),
		new Option("选项Cgggggggg", "C", "10"),
	]);
</script>

<style lang="scss">
	// 引入全局样式
	@use "@/assets/style/default/index.scss" as *;

	// 应用容器样式
	#app {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 100vh; // 使用视口高度填充全屏
		max-height: 100vh;
		background-color: var(--bg-primary); // 使用 colours 变量
	}
</style>
