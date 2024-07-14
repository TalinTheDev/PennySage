<script setup>
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";

  const user = useUserStore();
  const router = useRouter();

  const logout = async () => {
    await user.logout();
    router.push({ name: "login" });
  };
</script>
<template>
  <h2>Dashboard</h2>
  <h3>Welcome {{ user.getUsername }}</h3>
  <button @click="logout">Logout</button>
</template>
