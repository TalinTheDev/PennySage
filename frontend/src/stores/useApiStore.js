import axios from "axios";
import { ref } from "vue";
import { defineStore } from "pinia";
import { useUserStore } from "./useUserStore";
import { useRouter } from "vue-router";
export const useApiStore = defineStore("useApiStore", () => {
  const user = useUserStore();
  const router = useRouter();

  let token = "";
  if (user.checkLoggedIn) {
    token = user.accessToken;
  }
  const axiosInstance = axios.create({
    baseURL: "http://127.0.0.1:3000/api/",
    withCredentials: true,
    responseType: "json",
    headers: {
      Accept: "application/json",
      "Content-type": "application/json",
      Authorization: `Bearer ${token}`
    }
  });

  axiosInstance.interceptors.response.use(
    (response) => response,
    (error) => {
      let loginUrl = error.config.url === error.config.baseURL + "/auth/login";
      let registerUrl = error.config.url === error.config.baseURL + "/auth/register";
      let refreshTokenUrl = error.config.url === error.config.baseURL + "/auth/refresh_token";
      if (loginUrl || registerUrl || refreshTokenUrl) {
        return;
      }
      if (user.checkLoggedIn && error.response.status === 401) {
        axiosInstance
          .post("/auth/refresh_tokens")
          .then(({ data }) => {
            user.login(data.user_details, data.access_token);
            let request = error.config;
            request.headers["Authorization"] = `Bearer ${data.access_token}`;
            axiosInstance
              .request(request)
              .then(({ data }) => {
                return Promise.resolve(data);
              })
              .catch((error) => {
                return Promise.reject(error);
              });
          })
          .catch((error) => {
            router.push("/login");
            return Promise.reject(error);
          });
      }
      return Promise.reject(error);
    }
  );

  const api = ref(axiosInstance);

  return { api };
});
