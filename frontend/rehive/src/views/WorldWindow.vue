<template>
	<div ref="threeCanvas"></div>
</template>

<script lang="ts">
	import {
		defineComponent,
		onMounted,
		onBeforeUnmount,
		ref,
		watch,
	} from "vue";
	import * as THREE from "three";
	import { FlyControls } from "three/examples/jsm/controls/FlyControls";
	import { useWebSocket, WebSocketConfig } from "@/hooks/useWebSocket.hook";
	import { createThreeScene, makeThreeRefs } from "@/utils/threejs";
	import hotkeys from "hotkeys-js";
	import { createCameraController } from "@/utils/threejs/createShortcuts";

	/**
	 * WorldWindow 组件
	 */
	export default defineComponent({
		name: "WorldWindow",

		//--------------------------------------------------
		// 1. Props
		//--------------------------------------------------
		props: {
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
		},

		setup(props) {
			//--------------------------------------------------
			// 2. refs, 变量声明
			//--------------------------------------------------
			const threeCanvas = ref<HTMLDivElement | null>(null); // 容器 DOM
			let renderer: THREE.WebGLRenderer | null = null;
			let camera: THREE.PerspectiveCamera | null = null;
			let scene: THREE.Scene | null = null;
			let controls: FlyControls | null = null;

			const dependencies = makeThreeRefs(
				scene,
				camera,
				renderer,
				controls
			);

			const { initializeScene, updateRendererSize, updateSceneWithData } =
				createThreeScene(
					threeCanvas,
					props.width,
					props.height,
					dependencies
				);

			// WebSocket 配置
			const wsConfig: WebSocketConfig = {
				url: "wss://your-backend-endpoint.com",
				autoReconnect: true,
				reconnectInterval: 2000,
				maxReconnectAttempts: 3,
				onOpen: (socket) => {
					socket.send(
						JSON.stringify({
							type: "fetch",
							id: props.civilization_id,
						})
					);
				},
				onMessage: (event) => {
					const data = JSON.parse(event.data);
					console.log("Received data:", data);
					updateSceneWithData(data);
				},
				onError: (error) => {
					console.error("WebSocket error:", error);
				},
				onClose: (event) => {
					console.log("WebSocket closed:", event);
				},
			};

			// 使用 WebSocket Hook
			const { connect, disconnect } = useWebSocket(wsConfig);

			//--------------------------------------------------
			// 3. 生命周期钩子
			//--------------------------------------------------
			onMounted(() => {
				// 初始化场景、渲染器以及 Controls
				const initThreeRefs = initializeScene();

				// 模拟获取数据后更新场景并获取相机默认信息
				const sceneInfo = mockFetchData();

				// 检查是否成功获取场景信息
				if (!sceneInfo || !initThreeRefs?.camera) {
					console.error("Failed to initialize scene data.");
					return;
				} else {
					// 解构场景信息
					const { defaultPosition, defaultLookAt } = sceneInfo;

					// 创建相机控制器
					const {
						resetCameraPosition,
						reverseCamera,
						updateCameraZ,
					} = createCameraController(
						initThreeRefs.camera, // 包含 scene, camera, renderer, controls
						defaultPosition,
						defaultLookAt
					);

					// Alt + G：重置相机位置
					hotkeys("alt+g", resetCameraPosition);

					// Alt + R：摄像头反转 180 度
					hotkeys("alt+r", reverseCamera);

					// Z + 数字 + Enter：更新相机 Z 坐标
					let zValueInput = "";
					hotkeys("z+*", (event) => {
						// 将 event.key 转为数字后使用 isNaN 检查
						if (!isNaN(Number(event.key))) {
							zValueInput += event.key; // 拼接数字
						}
					});

					hotkeys("enter", () => {
						if (zValueInput) {
							const zValue = parseFloat(zValueInput);
							updateCameraZ(zValue);
							zValueInput = ""; // 重置输入
						}
					});
				}
			});

			onBeforeUnmount(() => {
				// 清理 WebSocket 连接
				disconnect();
				// 清理 Controls
				if (dependencies.controls) {
					dependencies.controls.dispose();
				}
				hotkeys.unbind("alt+g");
				hotkeys.unbind("alt+r");
				hotkeys.unbind("z+*");
				hotkeys.unbind("enter");
			});

			//--------------------------------------------------
			// 4. Watchers
			//--------------------------------------------------
			watch(
				() => [props.width, props.height],
				() => {
					updateRendererSize();
				}
			);

			//--------------------------------------------------
			// 5. 主要逻辑方法
			//--------------------------------------------------

			/**
			 * 模拟获取数据后更新场景
			 * @returns 更新场景后的信息（defaultPosition 和 defaultLookAt）
			 */
			const mockFetchData = () => {
				// 模拟获取数据并更新场景
				const sceneUpdateResult = updateSceneWithData({
					rowNum: 256,
					columnNum: 256,
				});

				// 检查返回值是否存在
				if (sceneUpdateResult) {
					const { defaultPosition, defaultLookAt } =
						sceneUpdateResult;

					console.log("Default Position:", defaultPosition);
					console.log("Default LookAt:", defaultLookAt);

					// 返回更新后的信息
					return { defaultPosition, defaultLookAt };
				} else {
					console.error("Failed to update scene with data.");
					return null; // 返回 null 表示更新失败
				}
			};

			//--------------------------------------------------
			// 7. 返回给模板使用的变量/方法
			//--------------------------------------------------
			return {
				threeCanvas,
			};
		},
	});
</script>

<style lang="scss">
	/* 根据需要编写样式 */
</style>
