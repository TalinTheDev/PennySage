<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import { ref } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";

  const transactions = ref("");
  const api = useApiStore().api;

  api
    .get("/transactions/all")
    .then(({ data }) => {
      transactions.value = data;
    })
    .catch((error) => {
      alert(`Error: ${JSON.stringify(error)}`);
    });
</script>
<template>
  <AppLayout>
    <h2>View Transactions</h2>
    {{ transactions }}
  </AppLayout>
</template>
