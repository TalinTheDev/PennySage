<script setup>
  const props = defineProps(["title"]);
</script>
<template>
  <span class="header">{{ props.title }}</span>
  <slot />
</template>

<style scoped>
  .header {
    text-decoration: none;
    font-size: 0.9rem;
    font-weight: bold;
    color: var(--neutral-500);
    position: relative;
    left: -5%;

    margin: 0 auto;
    padding: 5px 10px;
    border-radius: 10px;
  }
</style>
