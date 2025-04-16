import axios from "axios";
import { ref } from "vue";
import { defineStore } from "pinia";
import { useUserStore } from "./useUserStore";
import { useRouter } from "vue-router";

export const useApiStore = defineStore("useApiStore", () => {
  const user = useUserStore();
  const router = useRouter();
  const baseURL = "/api";
  const api = ref(
    axios.create({
      baseURL: baseURL,
      responseType: "json",
      withCredentials: true,
      headers: {
        Accept: "application/json",
        "Content-type": "application/json"
      }
    })
  );

  const refreshApiInstance = axios.create({
    baseURL: baseURL,
    responseType: "json",
    withCredentials: true,
    headers: {
      Accept: "application/json",
      "Content-type": "application/json"
    }
  });

  api.value.interceptors.request.use(
    (config) => {
      if (user.checkLoggedIn) {
        config.headers.Authorization = `Bearer ${user.accessToken}`;
      }
      return config;
    },
    (error) => {
      return Promise.reject(error.response.data);
    }
  );

  api.value.interceptors.response.use(
    (response) => {
      return response.data;
    },
    (error) => {
      let loginUrl = error.config.url === error.config.baseURL + "/auth/login";
      let registerUrl = error.config.url === error.config.baseURL + "/auth/register";
      let refreshTokenUrl = error.config.url === error.config.baseURL + "/auth/refresh_token";
      if (
        loginUrl ||
        registerUrl ||
        refreshTokenUrl ||
        !user.checkLoggedIn ||
        error.response.status !== 401
      ) {
        console.log("not refreshing token");
        return Promise.reject(error.response.data);
      }
      return new Promise((resolve, reject) => {
        refreshApiInstance
          .post("/auth/refresh_tokens")
          .then(({ data }) => {
            console.log("refreshed token");
            user.login(data.data.user_details, data.data.access_token);
            let request = error.config;
            api.value
              .request(request)
              .then((response) => {
                return resolve(response);
              })
              .catch((error) => {
                return reject(error);
              });
          })
          .catch((error) => {
            router.push("/login");
            return reject(error.response.data);
          });
      });
    }
  );

  return { api };
});
