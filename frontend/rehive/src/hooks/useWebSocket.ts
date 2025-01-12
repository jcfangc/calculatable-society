import { ref, onBeforeUnmount, onMounted } from "vue";

/**
 * WebSocket 配置接口
 * @property {string} url WebSocket 连接地址
 * @property {(socket: WebSocket) => void} [onOpen] WebSocket 打开时的回调
 * @property {(event: MessageEvent) => void} [onMessage] WebSocket 收到消息时的回调
 * @property {(event: Event) => void} [onError] WebSocket 错误时的回调
 * @property {(event: CloseEvent) => void} [onClose] WebSocket 关闭时的回调
 * @property {boolean} [autoReconnect=true] 是否自动重连
 * @property {number} [reconnectInterval=1000] 自动重连的时间间隔（毫秒）
 * @property {number} [maxReconnectAttempts=5] 最大重连次数
 * @property {boolean} [autoConnect=true] 是否自动连接
 */
interface WebSocketConfig {
	url: string; // WebSocket 连接地址
	onOpen?: (socket: WebSocket) => void; // WebSocket 打开时的回调
	onMessage?: (event: MessageEvent) => void; // 收到消息时的回调
	onError?: (event: Event) => void; // WebSocket 错误的回调
	onClose?: (event: CloseEvent) => void; // WebSocket 关闭时的回调
	autoReconnect?: boolean; // 是否自动重连
	reconnectInterval?: number; // 自动重连的时间间隔（ms）
	maxReconnectAttempts?: number; // 最大重连次数
	autoConnect?: boolean;
}

/**
 * 创建 WebSocket 配置对象
 * @param {string} url WebSocket 连接地址
 * @param {(socket: WebSocket) => void} [onOpen] WebSocket 打开时的回调
 * @param {(event: MessageEvent) => void} [onMessage] WebSocket 收到消息时的回调
 * @param {(event: Event) => void} [onError] WebSocket 错误时的回调
 * @param {(event: CloseEvent) => void} [onClose] WebSocket 关闭时的回调
 * @param {boolean} [autoReconnect=true] 是否自动重连
 * @param {number} [reconnectInterval=1000] 自动重连的时间间隔（毫秒）
 * @param {number} [maxReconnectAttempts=5] 最大重连次数
 * @param {boolean} [autoConnect=true] 是否自动连接
 * @returns {WebSocketConfig} WebSocket 配置对象
 */
export function makeWebSocketConfig(
	url: string,
	onOpen?: (socket: WebSocket) => void,
	onMessage?: (event: MessageEvent) => void,
	onError?: (event: Event) => void,
	onClose?: (event: CloseEvent) => void,
	autoReconnect: boolean = true,
	reconnectInterval: number = 1000,
	maxReconnectAttempts: number = 5,
	autoConnect: boolean = true
): WebSocketConfig {
	return {
		url,
		onOpen,
		onMessage,
		onError,
		onClose,
		autoReconnect,
		reconnectInterval,
		maxReconnectAttempts,
		autoConnect,
	};
}

/**
 * WebSocket Hook，管理 WebSocket 连接、消息、重连等功能
 *
 * @param config 配置对象，包含 WebSocket 的连接地址、回调函数和重连设置
 * @returns {Object} WebSocket 状态和控制方法
 * @returns {Ref<WebSocket | null>} socket 当前 WebSocket 实例，`null` 表示未连接
 * @returns {Function} connect 连接 WebSocket
 * @returns {Function} disconnect 断开 WebSocket 连接
 */
export function useWebSocket(config: WebSocketConfig) {
	const socket = ref<WebSocket | null>(null); // WebSocket 实例
	let reconnectAttempts = 0; // 当前重连尝试次数
	let reconnectTimer: ReturnType<typeof setTimeout> | null = null; // 重连定时器

	/**
	 * 连接 WebSocket
	 * 初始化 WebSocket 实例并设置事件处理程序。
	 */
	const connect = () => {
		if (socket.value) return; // 避免重复连接

		socket.value = new WebSocket(config.url);

		socket.value.onopen = () => {
			console.log("WebSocket connected");
			reconnectAttempts = 0;
			config.onOpen?.(socket.value!);
		};

		socket.value.onmessage = (event) => {
			console.log("WebSocket received message:", event.data);
			config.onMessage?.(event);
		};

		socket.value.onerror = (error) => {
			console.error("WebSocket error:", error);
			config.onError?.(error);

			// 自动重连
			if (config.autoReconnect) {
				reconnect();
			}
		};

		socket.value.onclose = (event) => {
			console.log("WebSocket connection closed");
			config.onClose?.(event);

			// 自动重连
			if (config.autoReconnect) {
				reconnect();
			}
		};
	};

	/**
	 * 尝试重连
	 */
	const reconnect = () => {
		if (
			reconnectAttempts >= (config.maxReconnectAttempts || Infinity) ||
			reconnectTimer
		) {
			return;
		}

		reconnectAttempts++;
		reconnectTimer = setTimeout(() => {
			console.log(
				`WebSocket reconnecting... (attempt ${reconnectAttempts})`
			);
			reconnectTimer = null;
			connect();
		}, config.reconnectInterval || 1000);
	};

	/**
	 * 断开连接
	 */
	const disconnect = () => {
		if (socket.value) {
			socket.value.close();
			socket.value = null;
		}
		if (reconnectTimer) {
			clearTimeout(reconnectTimer);
			reconnectTimer = null;
		}
	};

	// 在组件挂载阶段，自动建立连接（如果 autoConnect 为 true）
	onMounted(() => {
		if (config.autoConnect) {
			connect();
		}
	});

	// 在组件卸载阶段，断开连接
	onBeforeUnmount(() => {
		disconnect();
	});

	return {
		socket,
		connect,
		disconnect,
	};
}
