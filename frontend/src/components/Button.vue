<template>
  <button>
    <slot />
  </button>
</template>

<style scoped>
  button {
    background-color: var(--accent);
    color: var(--white);
    width: 100%;
    padding: 0.5rem;
    margin-top: 10px;
    border: none;
    height: min-content;
    border-radius: 6px;

    &:hover {
      background-color: var(--accent-hover);
      cursor: pointer;
    }
  }
</style>
