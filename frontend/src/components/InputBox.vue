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
    margin: 1% 0;
    width: 100%;
    display: block;

    .errorMsg {
      color: red;
      font-size: 0.8rem;
      display: block;
      text-align: start;

      &:not(.invalidLabel) {
        color: transparent;
      }
    }
  }
</style>
