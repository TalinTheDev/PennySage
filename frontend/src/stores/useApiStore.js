import axios from "axios";
import { ref } from "vue";
import { defineStore } from "pinia";
import { useUserStore } from "./useUserStore";
import { useRouter } from "vue-router";

export const useApiStore = defineStore("useApiStore", () => {
  const user = useUserStore();
  const router = useRouter();

  const api = ref(
    axios.create({
      baseURL: "http://192.168.1.33:3000/api/",
      responseType: "json",
      withCredentials: true,
      headers: {
        Accept: "application/json",
        "Content-type": "application/json"
      }
    })
  );

  const refreshApiInstance = axios.create({
    baseURL: "http://127.0.0.1:3000/api/",
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
        return Promise.reject(error.response.data);
      }
      return new Promise((resolve, reject) => {
        refreshApiInstance
          .post("/auth/refresh_tokens")
          .then(({ data }) => {
            user.login(data.user_details, data.access_token);
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
