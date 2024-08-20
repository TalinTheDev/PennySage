<script setup>
  import { ref } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";

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
    <PageHeader>
      <template #header>
        <span>Hey, {{ user.firstName }}</span>
      </template>
      <template #subheader>
        <span>{{ date }}</span>
      </template>
    </PageHeader>
  </AppLayout>
</template>

<style scoped>
  .header {
    width: 100vw;
    padding: 1rem 2rem;
    background-color: var(--white);
    box-shadow: 0px 3px 3px 0px var(--neutral-300);
    z-index: -1;
    position: relative;
    margin-bottom: 2rem;
  }
  .subheader {
    color: var(--neutral-500);
    font-size: 0.9rem;
    display: block;
  }
</style>
