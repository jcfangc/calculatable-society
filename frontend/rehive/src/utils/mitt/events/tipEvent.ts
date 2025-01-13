// Tip类型枚举
export const enum TipLevel {
	Success = "success",
	Warning = "warning",
	Error = "error",
	Info = "info",
}

// Tip事件枚举
export const enum TipEvent {
	Show = "tip:show",
}

// Tip的参数类型
export interface TipMessage {
	level: TipLevel;
	message: string;
}
