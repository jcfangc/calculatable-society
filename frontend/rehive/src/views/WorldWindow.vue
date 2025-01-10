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
			let socket: WebSocket | null = null;

			// 用来记录网格中心和默认的相机位置
			let globalCenterX: number;
			let globalCenterY: number;
			let defaultPosition: THREE.Vector3;
			let defaultLookAt: THREE.Vector3;

			//--------------------------------------------------
			// 3. 生命周期钩子
			//--------------------------------------------------
			onMounted(() => {
				initializeScene(); // 初始化场景、渲染器以及 Controls
				// connectWebSocket();     // 如果需要 WebSocket 功能，可解除注释
				mockFetchData(); // 开发阶段假装获取到数据并更新场景

				// 监听 Alt + G 快捷键，用于重置相机
				window.addEventListener("keydown", onAltGKeyDown);
			});

			onBeforeUnmount(() => {
				// 清理 WebSocket 连接
				if (socket) {
					socket.close();
					socket = null;
				}
				// 清理 Controls
				if (controls) {
					controls.dispose();
				}
				// 移除键盘事件监听
				window.removeEventListener("keydown", onAltGKeyDown);
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
			 * 初始化场景、相机、渲染器以及控制器
			 */
			const initializeScene = () => {
				if (!threeCanvas.value) return;

				const { scene: newScene, camera: newCamera } =
					createSceneAndCamera(props.width / props.height);
				scene = newScene;
				camera = newCamera;

				renderer = createRenderer(props.width, props.height);
				threeCanvas.value.appendChild(renderer.domElement);

				// 初始化 FlyControls
				controls = new FlyControls(camera, renderer.domElement);
				controls.movementSpeed = 10; // 移动速度
				controls.rollSpeed = Math.PI / 5; // 旋转速度
				controls.dragToLook = true; // 鼠标拖动来观察

				// 启动动画循环
				if (scene && camera && controls && renderer) {
					animate(scene, camera, renderer, controls);
				}
			};

			/**
			 * 在窗口大小变化时更新渲染器及相机
			 */
			const updateRendererSize = () => {
				if (renderer && camera) {
					renderer.setSize(props.width, props.height);
					camera.aspect = props.width / props.height;
					camera.updateProjectionMatrix();
				}
			};

			/**
			 * 连接并初始化 WebSocket（如果需要）
			 */
			const connectWebSocket = () => {
				socket = new WebSocket("wss://your-backend-endpoint.com");

				socket.onopen = () => {
					console.log("WebSocket connected");
					// 发送 civilization_id 请求
					socket?.send(
						JSON.stringify({
							type: "fetch",
							id: props.civilization_id,
						})
					);
				};

				socket.onmessage = (event) => {
					const data = JSON.parse(event.data);
					console.log("Received data:", data);
					updateSceneWithData(data);
				};

				socket.onerror = (error) => {
					console.error("WebSocket error:", error);
				};

				socket.onclose = () => {
					console.log("WebSocket connection closed");
				};
			};

			/**
			 * 根据数据更新场景
			 */
			const updateSceneWithData = (data: any) => {
				// 假定数据结构
				const mockData = {
					rowNum: 5,
					columnNum: 5,
					subtancetype: { numerator: 3, denominator: 7 },
				};

				if (!scene || !camera) return;

				// 清理现有对象
				clearSceneObjects(scene);

				// 基向量（六边形坐标->笛卡尔）
				const xBaseVector = { x: Math.sqrt(3) * 0.5, y: 0.5 };
				const yBaseVector = { x: 0.0, y: 1.0 };

				// 记录边界
				let minX = Infinity,
					minY = Infinity,
					maxX = -Infinity,
					maxY = -Infinity;

				// 生成六边形网格
				for (let row = 0; row < mockData.rowNum; row++) {
					for (let col = 0; col < mockData.columnNum; col++) {
						const centerX =
							col * xBaseVector.x + row * yBaseVector.x;
						const centerY =
							col * xBaseVector.y + row * yBaseVector.y;

						minX = Math.min(minX, centerX);
						minY = Math.min(minY, centerY);
						maxX = Math.max(maxX, centerX);
						maxY = Math.max(maxY, centerY);

						// 创建并添加六边形
						const hexagonMesh = createHexagonMesh(
							0.5, // 半径
							new THREE.Color(22 / 255, 0 / 255, 95 / 255),
							new THREE.Color(253 / 255, 48 / 255, 229 / 255),
							new THREE.Vector3(centerX, centerY, 0)
						);
						scene.add(hexagonMesh);
					}
				}

				// 网格中心坐标
				globalCenterX = (minX + maxX) / 2;
				globalCenterY = (minY + maxY) / 2;

				// 调整相机位置
				defaultPosition = new THREE.Vector3(
					globalCenterX,
					globalCenterY,
					Math.max(mockData.rowNum, mockData.columnNum)
				);
				camera.position.set(
					defaultPosition.x,
					defaultPosition.y,
					defaultPosition.z
				);

				defaultLookAt = new THREE.Vector3(
					globalCenterX,
					globalCenterY,
					0
				);
				camera.lookAt(defaultLookAt);

				// 使用 subtancetype 进行一些自定义处理
				console.log(
					`Subtype ratio: ${mockData.subtancetype.numerator}/${mockData.subtancetype.denominator}`
				);
			};

			/**
			 * 重置相机到默认位置
			 */
			const resetCameraPosition = () => {
				if (camera && defaultPosition && defaultLookAt) {
					camera.position.copy(defaultPosition);
					camera.lookAt(defaultLookAt);
					console.log("Camera reset to default position.");
				}
			};

			/**
			 * 模拟获取数据后更新场景
			 */
			const mockFetchData = () => {
				// 这里可以模拟异步请求，拿到数据后再更新
				updateSceneWithData({});
			};

			//--------------------------------------------------
			// 6. 事件处理函数
			//--------------------------------------------------

			/**
			 * 监听 Alt + G 快捷键，用于重置相机位置
			 */
			const onAltGKeyDown = (event: KeyboardEvent) => {
				if (event.altKey && event.key.toLowerCase() === "g") {
					resetCameraPosition();
				}
			};

			//--------------------------------------------------
			// 7. 辅助函数
			//--------------------------------------------------

			/**
			 * 创建场景和相机
			 */
			function createSceneAndCamera(aspect: number) {
				const scene = new THREE.Scene();
				const camera = new THREE.PerspectiveCamera(
					75,
					aspect,
					0.1,
					1000
				);
				camera.position.z = 5;
				return { scene, camera };
			}

			/**
			 * 创建渲染器
			 */
			function createRenderer(width: number, height: number) {
				const renderer = new THREE.WebGLRenderer({ alpha: true });
				renderer.setSize(width, height);
				return renderer;
			}

			/**
			 * 创建六边形网格单元
			 */
			function createHexagonMesh(
				radius: number = Math.sqrt(3) / 3,
				fillColor: THREE.ColorRepresentation = new THREE.Color(
					22 / 255,
					0 / 255,
					95 / 255
				),
				edgeColor: THREE.ColorRepresentation = new THREE.Color(
					253 / 255,
					48 / 255,
					229 / 255
				),
				position: THREE.Vector3 = new THREE.Vector3(0, 0, 0)
			): THREE.Mesh {
				// 几何体
				const hexagonGeometry = new THREE.CircleGeometry(radius, 6);

				// 边框
				const edges = new THREE.EdgesGeometry(hexagonGeometry);
				const edgeMaterial = new THREE.LineBasicMaterial({
					color: edgeColor,
				});
				const edgeLines = new THREE.LineSegments(edges, edgeMaterial);

				// 填充材质
				const material = new THREE.MeshBasicMaterial({
					color: fillColor,
				});
				const hexagonMesh = new THREE.Mesh(hexagonGeometry, material);

				// 将边框加到六边形上
				hexagonMesh.add(edgeLines);

				// 位置
				hexagonMesh.position.copy(position);

				return hexagonMesh;
			}

			/**
			 * 清空场景中的所有子对象
			 */
			function clearSceneObjects(scene: THREE.Scene) {
				while (scene.children.length > 0) {
					scene.remove(scene.children[0]);
				}
			}

			/**
			 * 动画循环
			 */
			function animate(
				scene: THREE.Scene,
				camera: THREE.PerspectiveCamera,
				renderer: THREE.WebGLRenderer,
				controls: FlyControls
			) {
				function loop() {
					requestAnimationFrame(loop);
					controls.update(0.016); // 更新控制器状态
					renderer.render(scene, camera);
				}
				loop();
			}

			//--------------------------------------------------
			// 8. 返回给模板使用的变量/方法
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
