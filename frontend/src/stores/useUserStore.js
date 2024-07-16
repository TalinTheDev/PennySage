import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { useApiStore } from "./useApiStore";
import { useRouter } from "vue-router";
import Cookies from "@/utils/Cookies.js";

export const useUserStore = defineStore("useUserStore", () => {
  const accessToken = ref(Cookies.get("accessToken"));

  const username = ref("");
  const email = ref("");
  const firstName = ref("");
  const lastName = ref("");
  const role = ref("");

  const login = (user, token) => {
    accessToken.value = token;
    username.value = user.username;
    email.value = user.email;
    firstName.value = user.firstName;
    lastName.value = user.lastName;
    role.value = user.role;

    Cookies.set("username", user.username);
    Cookies.set("role", user.role);
    Cookies.set("accessToken", token);
  };

  const logout = async () => {
    let { api } = useApiStore();
    username.value = "";
    email.value = "";
    firstName.value = "";
    lastName.value = "";
    role.value = "";

    accessToken.value = "";
    Cookies.delete("username");
    Cookies.delete("role");
    Cookies.delete("accessToken");
    Cookies.delete("retryingAuth");

    await api.post("/auth/logout");
    api.interceptors.response.clear();
  };

  const checkLoggedIn = computed(() => {
    console.log("Checking logged in");
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
