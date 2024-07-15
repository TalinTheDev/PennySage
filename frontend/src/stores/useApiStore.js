import axios from "axios";
import { ref } from "vue";
import { defineStore } from "pinia";
import { useUserStore } from "./useUserStore";

export const useApiStore = defineStore("useApiStore", () => {
  const user = useUserStore();
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

  const api = ref(axiosInstance);

  return { api };
});
