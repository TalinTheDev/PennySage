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
    let { api } = useApiStore();

    username.value = user.username;
    email.value = user.email;
    firstName.value = user.firstName;
    lastName.value = user.lastName;
    role.value = user.role;

    accessToken.value = token;
    Cookies.set("username", user.username);
    Cookies.set("role", user.role);

    api.interceptors.request.use((config) => {
      if (user.checkLoggedIn) {
        config.headers.Authorization = `Bearer ${user.getAccessToken}`;
      }
      return config;
    });

    api.interceptors.response.use(
      (response) => {
        return response;
      },
      (error) => {
        if (user.checkLoggedIn && error.response.status == 401) {
          api
            .post("/auth/refresh_tokens")
            .then((data) => {
              user.login(data.data.user_details, data.data.access_token);
            })
            .catch((error) => {
              return Promise.reject(error);
            });
        }
      }
    );
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

    await api.post("/auth/logout");
    api.interceptors.request.clear();
    api.interceptors.response.clear();
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
