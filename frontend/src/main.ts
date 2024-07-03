import "./style.css";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";
import { plugin, defaultConfig } from "@formkit/vue";

createApp(App)
  .use(createPinia())
  .use(router)
  .use(plugin, defaultConfig)
  .mount("#app");
