<template>
  <div class="cardParent"><slot /></div>
</template>

<style scoped>
  .cardParent {
    display: inline-block;
    border-radius: 10px;
    padding: 1.5rem;
    background-color: var(--white);
    border: 1px solid var(--neutral-300);
    width: max-content;
    margin: 1rem;
  }
</style>
