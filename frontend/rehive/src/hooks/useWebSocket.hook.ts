import { ref, onBeforeUnmount } from "vue";

/**
 * WebSocket 配置接口
 * @property {string} url WebSocket 连接地址
 * @property {(socket: WebSocket) => void} [onOpen] WebSocket 打开时的回调
 * @property {(event: MessageEvent) => void} [onMessage] WebSocket 收到消息时的回调
 * @property {(event: Event) => void} [onError] WebSocket 错误时的回调
 * @property {(event: CloseEvent) => void} [onClose] WebSocket 关闭时的回调
 * @property {boolean} [autoReconnect=true] 是否自动重连
 * @property {number} [reconnectInterval=1000] 自动重连的时间间隔（毫秒）
 * @property {number} [maxReconnectAttempts=Infinity] 最大重连次数
 */
export interface WebSocketConfig {
	url: string; // WebSocket 连接地址
	onOpen?: (socket: WebSocket) => void; // WebSocket 打开时的回调
	onMessage?: (event: MessageEvent) => void; // 收到消息时的回调
	onError?: (event: Event) => void; // WebSocket 错误的回调
	onClose?: (event: CloseEvent) => void; // WebSocket 关闭时的回调
	autoReconnect?: boolean; // 是否自动重连
	reconnectInterval?: number; // 自动重连的时间间隔（ms）
	maxReconnectAttempts?: number; // 最大重连次数
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

		socket.value = new WebSocket(config.url); // 创建 WebSocket 实例

		// WebSocket 打开时的处理
		socket.value.onopen = () => {
			console.log("WebSocket connected");
			reconnectAttempts = 0; // 重置重连计数
			config.onOpen?.(socket.value!); // 调用 onOpen 回调
		};

		// WebSocket 收到消息时的处理
		socket.value.onmessage = (event) => {
			console.log("WebSocket received message:", event.data);
			config.onMessage?.(event); // 调用 onMessage 回调
		};

		// WebSocket 错误时的处理
		socket.value.onerror = (error) => {
			console.error("WebSocket error:", error);
			config.onError?.(error); // 调用 onError 回调

			// 自动重连逻辑
			if (config.autoReconnect) {
				reconnect();
			}
		};

		// WebSocket 关闭时的处理
		socket.value.onclose = (event) => {
			console.log("WebSocket connection closed");
			config.onClose?.(event); // 调用 onClose 回调

			// 自动重连逻辑
			if (config.autoReconnect) {
				reconnect();
			}
		};
	};

	/**
	 * 尝试重连 WebSocket
	 * 只有在重连次数小于最大重连次数且没有定时器时才会重连
	 */
	const reconnect = () => {
		if (
			reconnectAttempts >= (config.maxReconnectAttempts || Infinity) || // 超过最大重连次数
			reconnectTimer // 已经有重连定时器
		) {
			return;
		}

		reconnectAttempts++; // 增加重连次数
		reconnectTimer = setTimeout(() => {
			console.log(
				`WebSocket reconnecting... (attempt ${reconnectAttempts})`
			);
			reconnectTimer = null;
			connect(); // 发起重连
		}, config.reconnectInterval || 1000); // 使用配置的间隔时间或默认 1 秒
	};

	/**
	 * 断开 WebSocket 连接
	 * 清理 WebSocket 实例和重连定时器
	 */
	const disconnect = () => {
		if (socket.value) {
			socket.value.close(); // 关闭 WebSocket 连接
			socket.value = null; // 清除 WebSocket 实例
		}
		if (reconnectTimer) {
			clearTimeout(reconnectTimer); // 清除重连定时器
			reconnectTimer = null;
		}
	};

	// 组件卸载时断开连接
	onBeforeUnmount(() => {
		disconnect();
	});

	// 返回 WebSocket 实例和控制方法
	return {
		socket,
		connect,
		disconnect,
	};
}
