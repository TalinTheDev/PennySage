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
    padding: 1rem;
    background-color: var(--white);
  }
  .header > * {
    font-weight: bold;
    font-size: 1.25rem;
    display: block;
  }
  .subheader > * {
    color: var(--light-grey);
    font-size: 0.9rem;
    display: block;
  }
</style>
