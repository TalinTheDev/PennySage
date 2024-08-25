<script setup>
  import { ref, inject, watchEffect } from "vue";

  const props = defineProps(["name"]);
  const errors = inject("errors");
  const error = ref(errors.value.get(`${props.name}Err`));

  watchEffect(() => {
    error.value = errors.value.get(`${props.name}Err`);
  });
</script>

<template>
  <div class="inputField">
    <slot />
    <span class="errorMsg" :class="{ invalidLabel: error }">{{ error }}&nbsp;</span>
  </div>
</template>

<style scoped>
  .inputField {
    width: 100%;
    margin: 0.5em 0;
    display: block;

    .errorMsg {
      color: var(--error-text);
      font-size: 0.7rem;
      display: block;
      position: relative;
      top: 0.2rem;
      text-align: start;

      &:not(.invalidLabel) {
        color: transparent;
      }
    }
  }
</style>
