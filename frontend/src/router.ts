import { createWebHistory, createRouter } from "vue-router";

import HomeView from "./views/HomeView.vue";
import LoginView from "./views/LoginView.vue";
import RegisterView from "./views/RegisterView.vue";
import AddTransactionView from "./views/AddTransactionView.vue";
import ViewTransactionsView from "./views/ViewTransactionsView.vue";
import { useUserStore } from "./stores/useUserStore";

const routes = [
  { path: "/", component: HomeView, name: "Home" },
  { path: "/login", component: LoginView, name: "Login" },
  { path: "/register", component: RegisterView, name: "Register" },
  { path: "/add", component: AddTransactionView, name: "Add Transaction" },
  { path: "/view", component: ViewTransactionsView, name: "View Transactions" }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

router.beforeEach(async (to) => {
  const userStore = useUserStore();
  if (!userStore.isLoggedIn && to.name !== "Login" && to.name !== "Register") {
    return { name: "Login" };
  }
});

export default router;
