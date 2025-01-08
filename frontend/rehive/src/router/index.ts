import { createRouter, createWebHistory } from "vue-router";

// 创建路由对象
const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: "/login",
			name: "登录页面",
			component: () => import("@/views/Login.vue"),
		},
		{
			path: "/overview",
			name: "概览页面",
			component: () => import("@/views/Overview.vue"),
			meta: {
				requiresAuth: true,
			},
		},
		{
			path: "/main",
			name: "主要页面",
			component: () => import("@/views/Main.vue"),
			meta: {
				requiresAuth: true,
			},
		},
		{
			path: "/",
			redirect: "/overview",
		},
	],
});

router.beforeEach((to, _, next) => {
	if (to.meta.requiresAuth && !localStorage.getItem("token")) {
		next("/login"); // 未登录重定向
	} else {
		next(); // 放行
	}
});

export default router;
