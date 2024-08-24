<script setup lang="ts">
  import PageHeader from "@/components/PageHeader.vue";
</script>

<template>
  <div class="page">
    <PageHeader>
      <template #header> <h1>PennySage</h1></template>
      <template #subheader><span>Welcome!</span></template>
    </PageHeader>
    <div class="content">
      <slot />
    </div>
  </div>
</template>

<style scoped>
  .page {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .content {
    width: 35vw;
    margin: auto;
  }
</style>
