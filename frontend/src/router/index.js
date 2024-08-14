import { createRouter, createWebHistory } from "vue-router";
import LoginView from "@/views/LoginView.vue";
import DashboardView from "@/views/DashboardView.vue";
import AddView from "@/views/AddView.vue";
import ViewView from "@/views/ViewView.vue";
import SettingsView from "@/views/SettingsView.vue";
import test from "@/views/test.vue";

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
    },
    {
      path: "/test",
      name: "test",
      component: test
    },
    {
      path: "/add",
      name: "add",
      component: AddView
    },
    {
      path: "/view",
      name: "view",
      component: ViewView
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView
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
