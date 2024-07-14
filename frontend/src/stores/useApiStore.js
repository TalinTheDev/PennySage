import axios from "axios";
import { ref } from "vue";
import { defineStore } from "pinia";

export const useApiStore = defineStore("useApiStore", () => {
  const axiosInstance = axios.create({
    baseURL: "http://127.0.0.1:3000/api/",
    withCredentials: true,
    responseType: "json",
    headers: {
      Accept: "application/json",
      "Content-type": "application/json"
    }
  });

  const api = ref(axiosInstance);

  return { api };
});
