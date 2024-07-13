import axios from "axios";
import { ref } from "vue";
import { defineStore } from "pinia";
import { useUserStore } from "./useUserStore";

export const useApiStore = defineStore("useApiStore", () => {
  const api = ref(
    axios.create({
      baseURL: "http://127.0.0.1:3000/api/",
      withCredentials: true,
      responseType: "json",
      headers: {
        Accept: "application/json",
        "Content-type": "application/json",
        Authorization: `Bearer ${useUserStore().getAccessToken}`
      }
    })
  );

  return { api };
});
