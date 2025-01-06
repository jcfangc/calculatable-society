export const enum InputType {
	Text = "text", // 单行文本输入框
	Password = "password", // 密码输入框
	Number = "number", // 数字输入框
	Email = "email", // 邮箱输入框
	Tel = "tel", // 电话号码输入框
	Url = "url", // URL输入框
	Date = "date", // 日期输入框
	Time = "time", // 时间输入框
	DatetimeLocal = "datetime-local", // 本地日期时间输入框
	Month = "month", // 月份选择框
	Week = "week", // 周选择框
	Color = "color", // 颜色选择框
	File = "file", // 文件上传框
	Checkbox = "checkbox", // 复选框
	Radio = "radio", // 单选按钮
	Range = "range", // 滑动选择框
	Search = "search", // 搜索框
	Hidden = "hidden", // 隐藏输入框
	Submit = "submit", // 提交按钮
	Reset = "reset", // 重置按钮
	Button = "button", // 普通按钮
	Image = "image", // 图片按钮
}

export const enum InputEvent {
	// 输入事件
	Input = "input", // 输入框内容改变时触发
	Change = "change", // 输入框内容改变后失去焦点时触发
	CompositionStart = "compositionstart", // 输入法开始输入
	CompositionUpdate = "compositionupdate", // 输入法输入更新
	CompositionEnd = "compositionend", // 输入法输入结束

	// 焦点事件
	Focus = "focus", // 输入框获得焦点
	Blur = "blur", // 输入框失去焦点
	FocusIn = "focusin", // 输入框获得焦点，包括子元素
	FocusOut = "focusout", // 输入框失去焦点，包括子元素

	// 键盘事件
	KeyDown = "keydown", // 按键按下
	KeyPress = "keypress", // 按键按下（已废弃，不推荐使用）
	KeyUp = "keyup", // 按键释放

	// 剪贴板事件
	Cut = "cut", // 剪切内容
	Copy = "copy", // 复制内容
	Paste = "paste", // 粘贴内容

	// 鼠标事件
	Click = "click", // 点击事件
	DblClick = "dblclick", // 双击事件
	MouseDown = "mousedown", // 鼠标按下
	MouseUp = "mouseup", // 鼠标释放
	MouseMove = "mousemove", // 鼠标移动

	// 拖拽事件
	DragEnter = "dragenter", // 拖入输入框区域
	DragOver = "dragover", // 拖拽经过输入框区域
	Drop = "drop", // 拖拽释放文件或数据

	// 表单事件
	Submit = "submit", // 提交表单
	Reset = "reset", // 重置表单

	// 上下文菜单事件
	ContextMenu = "contextmenu", // 右键菜单事件
}
