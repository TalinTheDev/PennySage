<script setup>
  import { ref } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";

  import { useUserStore } from "@/stores/useUserStore.js";
  import { useApiStore } from "@/stores/useApiStore.js";

  const api = useApiStore().api;
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

  api.get("/analytics/dashboard").then((response) => {
    console.log(response);
  });
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
    box-shadow: 0px 3px 3px 0px var(--border);
    z-index: -1;
    position: relative;
    margin-bottom: 2rem;
  }
  .subheader {
    color: var(--light-grey);
    font-size: 0.9rem;
    display: block;
  }
</style>
