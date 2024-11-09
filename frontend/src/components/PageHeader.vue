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
    padding: 1.5rem 1rem 0.6rem;
    background-color: var(--white);
  }
  .header > * {
    font-weight: bold;
    font-size: 1.5rem;
    display: block;
  }
  .subheader > * {
    color: var(--light-grey);
    font-size: 1rem;
    display: block;
  }
</style>
