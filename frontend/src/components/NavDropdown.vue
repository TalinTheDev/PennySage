<script setup>
  const props = defineProps(["title"]);
</script>
<template>
  <div class="linkGroup">
    <span class="header">{{ props.title }}</span>
    <slot />
  </div>
</template>

<style scoped>
  .linkGroup {
    margin-bottom: 1rem;
  }
  .header {
    text-decoration: none;
    font-size: 0.9rem;
    font-weight: bold;
    color: var(--neutral-700);
    position: relative;
    left: -5%;

    margin: 0 auto;
    padding: 5px 10px;
    border-radius: 10px;
  }
</style>
