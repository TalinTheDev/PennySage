import { createRouter, createWebHistory } from "vue-router";
import LoginView from "@/views/LoginView.vue";
import DashboardView from "@/views/DashboardView.vue";
import { useUserStore } from "@/stores/useUserStore";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      name: "login",
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

router.beforeEach((to, from) => {
  let user = useUserStore();

  if (
    from.path !== "/login" &&
    from.path !== "/register" &&
    to.name !== "login" &&
    to.name !== "register" &&
    !user.checkLoggedIn
  ) {
    return { name: "login" };
  }
});

export default router;
