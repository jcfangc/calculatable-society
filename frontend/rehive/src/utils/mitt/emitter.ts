import mitt from "mitt";
import { TipMessage, TipEvent } from "./events/tipEvent"; // 确保路径正确

// 定义事件类型
type Events = {
	[TipEvent.Show]: TipMessage; // 显式定义 TipEvent.Show 的参数类型为 TipMessage
};

export const emitter = mitt<Events>();
