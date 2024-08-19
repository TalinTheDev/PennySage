<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";

  import { useUserStore } from "@/stores/useUserStore.js";
  const user = useUserStore();
</script>
<template>
  <AppLayout>
    <h3>Welcome {{ user.firstName }}</h3>
  </AppLayout>
</template>
