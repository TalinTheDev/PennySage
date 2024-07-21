<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";

  import { useUserStore } from "@/stores/useUserStore.js";
  const user = useUserStore();
</script>
<template>
  <AppLayout>
    <h2>Dashboard</h2>
    <h3>Welcome {{ user.getUsername }}</h3>
  </AppLayout>
</template>
