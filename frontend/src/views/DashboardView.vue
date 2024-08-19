<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";

  import { useUserStore } from "@/stores/useUserStore.js";
  const user = useUserStore();
</script>
<template>
  <AppLayout>
    <span class="header">Hey, {{ user.firstName }}</span>
  </AppLayout>
</template>

<style scoped>
  .header {
    font-weight: bold;
    font-size: 1.25rem;
    display: block;
  }
</style>
