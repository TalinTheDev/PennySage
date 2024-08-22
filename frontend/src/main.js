import "./styles/styles.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import Nora from "@primevue/themes/nora";
import App from "./App.vue";
import router from "./router";
import Cookies from "./utils/Cookies";
import { useUserStore } from "./stores/useUserStore";
import "primeicons/primeicons.css";

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);
app.use(router);
app.use(PrimeVue, {
  theme: {
    preset: Nora,
    options: {
      darkModeSelector: ".theme-dark",
      cssLayer: {
        name: "primevue",
        order: "reset, primevue"
      }
    }
  }
});

if (Cookies.check("accessToken")) {
  useUserStore().login(
    { username: Cookies.get("username"), role: Cookies.get("role") },
    Cookies.get("accessToken")
  );
}

app.mount("#app");
