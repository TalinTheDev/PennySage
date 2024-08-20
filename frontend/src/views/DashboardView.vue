<script setup>
  import { ref } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";

  import { useUserStore } from "@/stores/useUserStore.js";
  const user = useUserStore();

  let date = new Date();
  date = ref(
    new Intl.DateTimeFormat("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric"
    }).format(date)
  );
</script>
<template>
  <AppLayout>
    <h1>Hey, {{ user.firstName }}</h1>
    <span class="subheader">{{ date }}</span>
  </AppLayout>
</template>

<style scoped>
  .subheader {
    color: var(--neutral-500);
    font-size: 0.9rem;
    display: block;
  }
</style>
