import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { useApiStore } from "./useApiStore";
import Cookies from "@/utils/Cookies.js";

export const useUserStore = defineStore("useUserStore", () => {
  const accessToken = ref("");
  const getAccessToken = computed(() => accessToken.value);

  const username = ref("");
  const getUsername = computed(() => username.value);
  const email = ref("");
  const getEmail = computed(() => email.value);
  const firstName = ref("");
  const getFirstName = computed(() => firstName.value);
  const lastName = ref("");
  const getLastName = computed(() => lastName.value);
  const role = ref("");
  const getRole = computed(() => role.value);

  const login = (user, token) => {
    username.value = user.username;
    email.value = user.email;
    firstName.value = user.firstName;
    lastName.value = user.lastName;
    role.value = user.role;

    accessToken.value = token;
    Cookies.set("username", user.username);
    Cookies.set("role", user.role);
  };

  const logout = async () => {
    username.value = "";
    email.value = "";
    firstName.value = "";
    lastName.value = "";
    role.value = "";

    accessToken.value = "";
    Cookies.delete("username");
    Cookies.delete("role");

    await useApiStore().api.post("/auth/logout");
  };

  const checkLoggedIn = computed(() => {
    if (accessToken.value) {
      return true;
    }
    return Cookies.check("username");
  });

  return {
    getAccessToken,
    getUsername,
    getEmail,
    getFirstName,
    getLastName,
    getRole,
    login,
    logout,
    checkLoggedIn
  };
});
