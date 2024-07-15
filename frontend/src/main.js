import "./styles/styles.css";

import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";
import Cookies from "./utils/Cookies";
import { useUserStore } from "./stores/useUserStore";

const app = createApp(App);

app.use(createPinia());

if (Cookies.check("accessToken")) {
  useUserStore().login(
    { username: Cookies.get("username"), role: Cookies.get("role") },
    Cookies.get("accessToken")
  );
}

app.use(router);

app.mount("#app");
