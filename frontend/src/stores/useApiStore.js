import axios from "axios";
import { ref } from "vue";
import { defineStore } from "pinia";
import { useUserStore } from "./useUserStore";

export const useApiStore = defineStore("useApiStore", () => {
  const user = useUserStore();
  const axiosInstance = axios.create({
    baseURL: "http://127.0.0.1:3000/api/",
    withCredentials: true,
    responseType: "json",
    headers: {
      Accept: "application/json",
      "Content-type": "application/json"
    }
  });

  axiosInstance.interceptors.request.use((config) => {
    if (user.checkLoggedIn) {
      config.headers.Authorization = `Bearer ${user.getAccessToken}`;
    }
    return config;
  });

  axiosInstance.interceptors.response.use(
    (response) => {
      return response;
    },
    (error) => {
      if (user.checkLoggedIn && error.response.status == 401) {
        axios
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

  const api = ref(axiosInstance);

  return { api };
});
