<template>
	<!-- 使用按钮组件 -->
	<div>
		<ReButton
			:onClickCallback="clickCallback"
			name="测试按钮"
			>test</ReButton
		>
	</div>
	<div>
		<ReInputBox
			v-model:input-value="inputValue"
			:onInputCallback="inputCallback"
			placeholder="请输入内容"
		></ReInputBox>
	</div>
	<div class="aaa">{{ inputValue }}</div>

	<div style="position: relative">
		<ReListbox
			:options="options"
			v-model:selected-option="selectedOption"
			:onSelectCallback="selectCallback"
		></ReListbox>
	</div>
	<div class="aaa">{{ selectedOption }}</div>
</template>

<script setup lang="ts">
	import { ref } from "vue";

	import ReButton from "./components/ReButton.vue";
	import ReInputBox from "./components/ReInput.vue";
	import ReListbox from "./components/ReListbox.vue";

	// 按钮回调
	const clickCallback = async (
		value: MouseEvent,
		abortSignal?: AbortSignal
	) => {
		await new Promise<void>((resolve, reject) => {
			console.log("点击回调", value);

			// 设定倒计时（3秒）
			let countdown = 3;

			const intervalId = setInterval(() => {
				if (abortSignal?.aborted) {
					clearInterval(intervalId); // 如果中止信号触发，停止倒计时
					reject(new DOMException("操作已被中止", "AbortError")); // 抛出AbortError
					return;
				}

				console.log(`${countdown}秒`);
				countdown--;

				// 当倒计时结束时清除定时器
				if (countdown < 0) {
					clearInterval(intervalId);
					resolve(); // 倒计时完成，调用resolve
				}
			}, 1000);
		});
	};

	const inputValue = ref("");
	// 输入回调
	const inputCallback = async (value: string, abortSignal?: AbortSignal) => {
		await new Promise<void>((resolve, reject) => {
			console.log("输入回调", value);

			// 设定倒计时（3秒）
			let countdown = 3;

			const intervalId = setInterval(() => {
				if (abortSignal?.aborted) {
					clearInterval(intervalId); // 如果中止信号触发，停止倒计时
					reject(new DOMException("操作已被中止", "AbortError")); // 抛出AbortError
					return;
				}

				console.log(`${countdown}秒`);
				countdown--;

				// 当倒计时结束时清除定时器
				if (countdown < 0) {
					clearInterval(intervalId);
					resolve(); // 倒计时完成，调用resolve
				}
			}, 1000);
		});
	};

	// 选项数据
	const options = [
		"选项1",
		"选项2",
		"选项3",
		"选项4",
		"选项5",
		"选项6",
		"选项7",
		"选项8",
		"选项9",
		"选项10",
		"选项11",
		"选项12",
		"选项13",
		"选项14",
		"选项15",
		"选项16",
		"选项17",
		"选项18",
		"选项19",
		"选项20",
	];
	const selectedOption = ref("");
	// 输入回调
	const selectCallback = async (
		value: string | number,
		abortSignal?: AbortSignal
	) => {
		await new Promise<void>((resolve, reject) => {
			console.log("选择回调", value);

			// 设定倒计时（3秒）
			let countdown = 3;

			const intervalId = setInterval(() => {
				if (abortSignal?.aborted) {
					clearInterval(intervalId); // 如果中止信号触发，停止倒计时
					reject(new DOMException("操作已被中止", "AbortError")); // 抛出AbortError
					return;
				}

				console.log(`${countdown}秒`);
				countdown--;

				// 当倒计时结束时清除定时器
				if (countdown < 0) {
					clearInterval(intervalId);
					resolve(); // 倒计时完成，调用resolve
				}
			}, 1000);
		});
	};
</script>

<style lang="scss">
	// 引入全局样式
	@use "@/assets/style/default/reset.scss" as *;

	// 应用容器样式
	#app {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 100vh; // 使用视口高度填充全屏
		max-height: 100vh;
		background-color: var(--bg-primary); // 使用 colours 变量
	}

	.aaa {
		color: red;
	}
</style>
