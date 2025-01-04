<template>
  <button
    class="btn"
    :type="type"
    :disabled="disabled || isLoading"
    :autofocus="autofocus"
    :name="name"
    :value="value"
    :aria-label="ariaLabel"
    :aria-labelledby="ariaLabelledby"
    :aria-pressed="isLoading"
    role="button"
    @click="handleClick"
  >
    <!-- 使用作用域插槽 -->
    <slot
      :isLoading="isLoading"
      :disabled="disabled"
      :type="type"
      :autofocus="autofocus"
      :value="value"
      :name="name"
    >
      <!-- 默认内容 -->
      <span>按钮</span>
    </slot>
  </button>
</template>

<script lang="ts" setup>
// 导入全局枚举
import { ButtonType } from "@/enums/button.enum";

// 定义 Props
defineProps({
  // 按钮类型
  type: {
    type: String as () => ButtonType,
    default: ButtonType.Button,
  },
  // 禁用状态
  disabled: {
    type: Boolean,
    default: false,
  },
  // 自动聚焦
  autofocus: {
    type: Boolean,
    default: false,
  },
  // 名称
  name: {
    type: String,
    default: "",
  },
  // 提交值
  value: {
    type: String,
    default: "",
  },
  // ARIA 属性
  ariaLabel: {
    type: String,
    default: "按钮，这是一个按钮！", // 提供额外的说明
  },
  ariaLabelledby: {
    type: String,
    default: "没有这个按钮用途的描述，看来它真的很神秘！", // 关联说明文本 ID
  },
});

import { ref, defineEmits } from "vue";
import { ButtonEvent } from "@/enums/button.enum";

// 定义内部变量
const isLoading = ref(false); // 加载状态由内部管理

// 点击事件
const emit = defineEmits([ButtonEvent.Click]); // 定义事件传递
function handleClick(event: MouseEvent) {
  if (!isLoading.value) {
    // 开始加载
    isLoading.value = true;

    // do something here

    // 抛出点击事件
    emit(ButtonEvent.Click, event);

    // 模拟加载完成后关闭状态（真实场景应通过异步操作关闭）
    setTimeout(() => {
      isLoading.value = false;
    }, 2000);
  }
}
</script>

<style scoped lang="scss">
@use "@/assets/style/default/index.scss" as *;

.btn {
  // 布局配置
  @include layout-config();

  // 排版配置
  @include typography-config();

  // 颜色配置
  @include colour-config();

  // 效果配置
  @include effects-config();

  cursor: pointer; // 光标样式
}
</style>
