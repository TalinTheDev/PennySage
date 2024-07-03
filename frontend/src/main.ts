import "./style.css";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";
import axios from "./plugins/axios";
import { useUserStore } from "./stores/useUserStore";

let app = createApp(App);
app.use(createPinia()).use(router).mount("#app");

let userStore = useUserStore();
app.provide("userStore", userStore);
app.provide(
  "api",
  axios.install(app, {
    baseUrl: "http://127.0.0.1:3000/api/",
    token: userStore.getToken()
  })
);
