<script setup>
  import { ref } from "vue";
  const props = defineProps(["title", "active"]);

  let active = ref(props.active || false);
  //let active = props.active || ref(false);
</script>
<template>
  <span class="header navLink" @click="active = !active">{{ props.title }}</span>
  <slot v-if="active" />
</template>

<style scoped>
  .header {
    font-weight: 500;
    text-decoration: none;
    color: var(--neutral-500);
    position: relative;
    left: -5%;

    margin: 0 auto;
    padding: 5px 10px;
    border-radius: 10px;
  }
</style>
