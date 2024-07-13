import { createRouter, createWebHistory } from "vue-router";
import LoginView from "@/views/LoginView.vue";
import DashboardView from "@/views/DashboardView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      name: "login",
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: LoginView
    },
    {
      path: "/register",
      name: "register",
      component: () => import("@/views/RegisterView.vue")
    },
    {
      path: "/",
      name: "dashboard",
      component: DashboardView
    }
  ]
});

export default router;
