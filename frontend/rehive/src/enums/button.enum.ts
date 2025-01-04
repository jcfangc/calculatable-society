// enums.ts - 全局常量枚举定义
export const enum ButtonType {
  Button = "button",
  Submit = "submit",
  Reset = "reset",
}

// src/enums/event.enum.ts
export const enum ButtonEvent {
  Click = "click", // 单击事件
  DblClick = "dblclick", // 双击事件
  MouseDown = "mousedown", // 鼠标按下
  MouseUp = "mouseup", // 鼠标释放
  MouseMove = "mousemove", // 鼠标移动
  MouseEnter = "mouseenter", // 鼠标进入元素区域
  MouseLeave = "mouseleave", // 鼠标离开元素区域
  MouseOver = "mouseover", // 鼠标悬浮到元素上（包含子元素）
  MouseOut = "mouseout", // 鼠标移出元素（包含子元素）
  ContextMenu = "contextmenu", // 鼠标右键点击，弹出菜单
}
