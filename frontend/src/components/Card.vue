<template>
  <div class="cardParent">
    <div class="header">
      <slot name="header" />
    </div>
    <div class="content">
      <slot name="content" />
    </div>
  </div>
</template>

<style scoped>
  .cardParent {
    display: inline-block;
    border-radius: 10px;
    background-color: var(--white);
    border: 1px solid var(--border);
    margin: 1rem;
    width: calc(100% - 2rem);

    .header > * {
      padding: 1rem 1rem;
      font-size: 1.2rem;
      font-weight: 500;
      width: 100%;
      display: block;
      border-bottom: 1px solid var(--border);
    }
    .content {
      padding: 1.5rem;
    }
  }
</style>
