import "./styles/styles.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";
import { definePreset } from "@primevue/themes";
import App from "./App.vue";
import router from "./router";
import Cookies from "./utils/Cookies";
import { useUserStore } from "./stores/useUserStore";
import "primeicons/primeicons.css";
import ToastService from "primevue/toastservice";
import ConfirmationService from "primevue/confirmationservice";

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);
app.use(router);
const MyPreset = definePreset(Aura, {
  semantic: {
    primary: {
      50: "{teal.50}",
      100: "{teal.100}",
      200: "{teal.200}",
      300: "{teal.300}",
      400: "{teal.400}",
      500: "{teal.500}",
      600: "{teal.600}",
      700: "{teal.700}",
      800: "{teal.800}",
      900: "{teal.900}",
      950: "{teal.950}"
    }
  }
});

app.use(PrimeVue, {
  theme: {
    preset: MyPreset,
    options: {
      darkModeSelector: ".theme-dark",
      cssLayer: {
        name: "primevue",
        order: "reset, primevue"
      }
    }
  }
});
app.use(ToastService);
app.use(ConfirmationService);
if (Cookies.check("accessToken")) {
  useUserStore().login(
    { username: Cookies.get("username"), role: Cookies.get("role") },
    Cookies.get("accessToken")
  );
}

app.mount("#app");
