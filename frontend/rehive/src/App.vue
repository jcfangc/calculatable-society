<template>
	<!-- <Sidebar /> -->
	<WorldWindow
		:width="windowWidth"
		:height="windowHeight"
		:civilization_id="civilizationId"
	/>
</template>

<script setup lang="ts">
	// import Sidebar from "@/components/Sidebar.vue";
	import WorldWindow from "@/views/WorldWindow.vue";
	import { ref, onMounted, onUnmounted } from "vue";
	// import { useDynForm } from "./hooks/useDynForm";
	// import { FormFieldType } from "./components/DynForm.vue";

	// useDynForm(
	// 	[
	// 		{
	// 			name: "name",
	// 			label: "Name",
	// 			type: FormFieldType.Text,
	// 			placeholder: "Enter your name",
	// 		},
	// 		{
	// 			name: "email",
	// 			label: "Email",
	// 			type: FormFieldType.Email,
	// 			placeholder: "Enter your email",
	// 		},
	// 		{
	// 			name: "password",
	// 			label: "Password",
	// 			type: FormFieldType.Password,
	// 			placeholder: "Enter your password",
	// 		},
	// 	],
	// 	(formData) => {
	// 		console.log(formData);
	// 	},
	// 	() => {
	// 		console.log("Cancelled");
	// 	}
	// );

	// 窗口宽高
	const windowWidth = ref(window.innerWidth);
	const windowHeight = ref(window.innerHeight);

	// 生成一个临时 UUID
	const civilizationId = ref(crypto.randomUUID());

	// 更新窗口大小
	const updateWindowSize = () => {
		windowWidth.value = window.innerWidth;
		windowHeight.value = window.innerHeight;
	};

	// 监听窗口大小变化
	onMounted(() => {
		window.addEventListener("resize", updateWindowSize);
	});

	// 移除监听器以避免内存泄漏
	onUnmounted(() => {
		window.removeEventListener("resize", updateWindowSize);
	});
</script>

<style lang="scss">
	// 引入全局样式
	@use "@/assets/style/default/_reset.scss" as *;

	// 应用容器样式
	#app {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		width: 100vw;
		height: 100vh;
		background: var(--bg-primary);
	}
</style>
