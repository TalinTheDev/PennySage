import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { useApiStore } from "./useApiStore";
import Cookies from "@/utils/Cookies.js";

export const useUserStore = defineStore("useUserStore", () => {
  const accessToken = ref(Cookies.get("accessToken"));

  const username = ref("");
  const email = ref("");
  const firstName = ref("");
  const lastName = ref("");
  const role = ref("");

  const login = (user, token) => {
    let { api } = useApiStore();

    username.value = user.username;
    email.value = user.email;
    firstName.value = user.firstName;
    lastName.value = user.lastName;
    role.value = user.role;

    Cookies.set("username", user.username);
    Cookies.set("role", user.role);
    Cookies.set("accessToken", token);

    api.interceptors.response.use(
      (response) => {
        return response;
      },
      (error) => {
        console.log("In interceptor: " + checkLoggedIn.value);
        if (checkLoggedIn.value && error.response.status == 401) {
          api
            .post("/auth/refresh_tokens")
            .then(({ data }) => {
              login(data.user_details, data.access_token);
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
    Cookies.delete("accessToken");

    await api.post("/auth/logout");
    api.interceptors.request.clear();
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
