import { createRouter, createWebHistory } from "vue-router";
import LoginView from "../views/LoginView.vue";
import HomeView from "../views/HomeView.vue";

const routes = [
  {
    path: "/login",
    name: "Login",
    component: LoginView,
  },
  {
    path: "/",
    name: "Home",
    component: HomeView,
    meta: { requiresAuth: true },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// 路由守卫：未登录时强制跳转到 /login
router.beforeEach((to, _from, next) => {
  const isLoggedIn = !!localStorage.getItem("user");
  if (to.meta.requiresAuth && !isLoggedIn) {
    next({ name: "Login" });
  } else if (to.name === "Login" && isLoggedIn) {
    // 已登录时访问 /login，直接跳回主页
    next({ name: "Home" });
  } else {
    next();
  }
});

export default router;
