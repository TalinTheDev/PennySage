<script setup>
  const props = defineProps(["type"]);
</script>

<template>
  <div class="credit" v-if="props.type == 'credit'">Credit</div>
  <div class="debit" v-else-if="props.type == 'debit'">Debit</div>
</template>

<style scoped>
  div {
    border-radius: 100px;
    padding: 0.25rem 1rem;
    font-size: 0.8rem;
    width: min-content;
  }

  .credit {
    background-color: var(--success-background);
    border: 2px solid var(--success-border);
    color: var(--success-text);
  }

  .debit {
    background-color: var(--error-background);
    border: 2px solid var(--error-border);
    color: var(--error-text);
  }
</style>
