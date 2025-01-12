import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vite.dev/config/
export default defineConfig({
	build: {
		cssCodeSplit: true, // 将 CSS 拆分为独立文件以便缓存和性能优化
		sourcemap: false, // 关闭 Source Map 加速构建
		rollupOptions: {
			output: {
				// 设置 JS 输出目录
				entryFileNames: "assets/js/[name].[hash].js",
				chunkFileNames: "assets/js/[name].[hash].js",
				assetFileNames: (assetInfo) => {
					const assetName = assetInfo.names[0] || "";
					if (assetName.endsWith(".css")) {
						return "assets/css/[name].[hash][extname]";
					}
					if (/\.(png|jpe?g|gif|svg|webp|ico)$/.test(assetName)) {
						return "assets/img/[name].[hash][extname]";
					}
					return "assets/[name].[hash][extname]";
				},
			},
		},
	},
	plugins: [vue()],
	resolve: {
		alias: {
			"@": fileURLToPath(new URL("./src", import.meta.url)),
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
});
