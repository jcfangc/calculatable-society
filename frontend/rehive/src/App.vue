<template>
  <div class="app-container">
    <!-- 使用按钮组件 -->
    <Button>提交</Button>
  </div>
</template>

<script setup lang="ts">
import Button from './components/Button.vue';
</script>

<style lang="scss">
// 引入全局样式
@use "@/assets/style/default/index.scss" as *;

// 应用容器样式
.app-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh; // 使用视口高度填充全屏
  padding: var(--spacing-md); // 使用 layout 变量
  background-color: var(--bg-primary); // 使用 colours 变量
}
</style>
