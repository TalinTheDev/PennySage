import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useUserStore = defineStore("user", () => {
  const access_token = ref("");

  const getToken = computed(() =>
    access_token.value === "" ? access_token.value : ""
  );
  const user = ref({
    username: "",
    email: "",
    firstName: "",
    lastName: ""
  });

  function logout() {
    access_token.value = "";
    user.value = {
      username: "",
      email: "",
      firstName: "",
      lastName: ""
    };
  }

  const isLoggedIn = computed(() => access_token.value !== "");

  return { getToken, user, logout, isLoggedIn };
});
