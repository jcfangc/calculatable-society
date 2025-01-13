<template>
	<div
		id="threeCanvas"
		ref="threeCanvas"
		@click="handleFocus"
		@focus="handleFocus"
		@blur="handleBlur"
	></div>
	<div id="crosshair">
		<div class="horizontal"></div>
		<div class="vertical"></div>
	</div>
</template>

<script setup lang="ts">
	import { onMounted, onBeforeUnmount, ref, Ref } from "vue";
	import { ThreeManager } from "@/utils/threejs/threeManager";
	import { ThreeCoreOptions } from "@/utils/threejs/threeCoreFactory";
	import { HexagonGridConfig } from "@/utils/threejs/hexagonGridManager";
	import { useWebSocket, makeWebSocketConfig } from "@/hooks/useWebSocket";

	const props = defineProps({
		width: {
			type: Number,
			required: true,
		},
		height: {
			type: Number,
			required: true,
		},
		civilization_id: {
			type: String,
			required: true,
			validator: (value: string) => /^[0-9a-fA-F-]{36}$/.test(value), // 验证是否为有效的 UUID
		},
	});

	const isActive = ref(false); // 是否激活当前组件
	const threeCanvas = ref<HTMLDivElement | null>(null); // 容器 DOM
	let jsonData: { rowNum: number; columnNum: number } = {
		rowNum: 128,
		columnNum: 128,
	}; // 初始网格数据

	function handleFocus() {
		isActive.value = true; // 设置激活状态
	}

	function handleBlur() {
		isActive.value = false; // 设置非激活状态
	}

	// WebSocket 配置
	const wsConfig = makeWebSocketConfig(
		"ws://this.is.a.websocket.url",
		() => {
			console.log("WebSocket 连接已建立");
		},
		(event) => {
			console.log("收到消息", event.data);
			// 解析并更新网格数据
			jsonData = JSON.parse(event.data);
		}
	);

	// const { socket, connect, disconnect } = useWebSocket(wsConfig); // WebSocket 实例

	// 实例化 ThreeManager
	let manager: ThreeManager | null = null;

	function initThreeManager() {
		if (!threeCanvas.value) return;

		// 配置 Three.js 核心选项
		const coreOptions: ThreeCoreOptions = {
			container: <Ref<HTMLDivElement>>threeCanvas, //
			width: props.width, // 视口宽度
			height: props.height, // 视口高度
			aspect: props.width / props.height, // 视角宽高比
			rendererOptions: {
				alpha: true, // 背景透明
				antialias: true, // 抗锯齿
			},
			mapDimensions: {
				rowNum: jsonData.rowNum, // 地图宽度
				columnNum: jsonData.columnNum, // 地图高度
			},
		};

		// 配置网格选项
		const gridConfig: Omit<HexagonGridConfig, "scene" | "camera"> = {
			rowNum: jsonData.rowNum, // 动态行数
			columnNum: jsonData.columnNum, // 动态列数
		};

		// 实例化 ThreeManager
		manager = new ThreeManager(coreOptions, isActive, gridConfig);
	}

	onMounted(() => {
		// connect(); // 建立 WebSocket 连接
		initThreeManager(); // 初始化 ThreeManager
	});

	onBeforeUnmount(() => {
		// disconnect(); // 断开 WebSocket 连接
		manager?.dispose(); // 清理 ThreeManager 实例
	});
</script>

<style lang="scss">
	#crosshair {
		position: fixed;
		top: 50%;
		left: 50%;
		width: 20px; // 准星的总宽度
		height: 20px; // 准星的总高度
		transform: translate(-50%, -50%);
		pointer-events: none; // 防止准星干扰鼠标事件

		// 水平线
		.horizontal,
		.vertical {
			position: absolute;
			background: white; // 准星的颜色
		}

		// 水平线样式
		.horizontal {
			top: 50%;
			left: 0;
			width: 100%;
			height: 2px; // 线条厚度
			transform: translateY(-50%);
		}

		// 垂直线样式
		.vertical {
			top: 0;
			left: 50%;
			width: 2px; // 线条厚度
			height: 100%;
			transform: translateX(-50%);
		}
	}
</style>
