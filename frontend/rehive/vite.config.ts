import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
// import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    // vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        // 自动引入全局变量和混合宏
        additionalData: `
          @use "@/assets/style/default/index.scss" as *;
        `,
      },
    },
  },
  server: {
    watch: {
      usePolling: true, // 使用轮询以提升监听精度（适合Docker或虚拟机环境）
    },
    hmr: true, // 启用热模块替换（Hot Module Replacement）
  },
  // build: {
  //   cssCodeSplit: true, // 将 CSS 拆分为独立文件以便缓存和性能优化
  //   sourcemap: false, // 关闭 Source Map 加速构建
  // },
})
