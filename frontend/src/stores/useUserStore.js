import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { useApiStore } from "./useApiStore";
import Cookies from "@/utils/Cookies.js";
import { useRouter } from "vue-router";

export const useUserStore = defineStore("useUserStore", () => {
  const { api } = useApiStore();
  const router = useRouter();
  const accessToken = ref(Cookies.get("accessToken"));

  const username = ref(Cookies.get("username"));
  const email = ref(Cookies.get("email"));
  const firstName = ref(Cookies.get("firstName"));
  const lastName = ref(Cookies.get("lastName"));
  const role = ref(Cookies.get("role"));

  const login = (user, token) => {
    accessToken.value = token;
    username.value = user.username;
    email.value = user.email || Cookies.get("email");
    firstName.value = user.first_name || Cookies.get("firstName");
    lastName.value = user.last_name || Cookies.get("lastName");
    role.value = user.role;

    Cookies.set("accessToken", token);
    Cookies.set("username", user.username);
    Cookies.set("email", user.email || Cookies.get("email"));
    Cookies.set("firstName", user.first_name || Cookies.get("firstName"));
    Cookies.set("lastName", user.last_name || Cookies.get("lastName"));
    Cookies.set("role", user.role);
  };

  async function logout() {
    username.value = "";
    email.value = "";
    firstName.value = "";
    lastName.value = "";
    role.value = "";

    accessToken.value = "";
    Cookies.delete("accessToken");
    Cookies.delete("username");
    Cookies.delete("email");
    Cookies.delete("firstName");
    Cookies.delete("lastName");
    Cookies.delete("role");
    Cookies.delete("retryingAuth");

    await api.post("/auth/logout");
    router.push({ name: "login" });
  }

  const checkLoggedIn = computed(() => {
    if (accessToken.value) {
      return true;
    }
    return Cookies.check("accessToken");
  });

  return {
    accessToken,
    username,
    email,
    firstName,
    lastName,
    role,
    login,
    logout,
    checkLoggedIn
  };
});
