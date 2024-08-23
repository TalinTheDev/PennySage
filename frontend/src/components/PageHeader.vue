<template>
  <div class="pageHeader">
    <div class="header">
      <slot name="header" />
    </div>
    <div class="subheader">
      <slot name="subheader" />
    </div>
  </div>
</template>

<style scoped>
  .pageHeader {
    padding: 1rem 2rem;
    background-color: var(--white);
    box-shadow: 0px 3px 3px 0px var(--neutral-300);
    z-index: -1;
    position: relative;
    margin-bottom: 1rem;
  }
  .header > * {
    font-weight: bold;
    font-size: 1.25rem;
    display: block;
  }
  .subheader > * {
    color: var(--neutral-500);
    font-size: 0.9rem;
    display: block;
  }
</style>
