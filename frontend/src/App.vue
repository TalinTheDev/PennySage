<script setup>
  import { RouterView } from "vue-router";
  import Toast from "primevue/toast";
  import ConfirmDialog from "primevue/confirmdialog";
</script>

<template>
  <Toast />
  <ConfirmDialog></ConfirmDialog>
  <RouterView />
</template>

<style scoped></style>
