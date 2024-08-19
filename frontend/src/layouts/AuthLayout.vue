<template>
  <div class="page">
    <div class="header">
      <h1>PennySage</h1>
    </div>
    <div class="content">
      <slot />
    </div>
  </div>
</template>

<style scoped>
  .page {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  .header {
    margin-top: 2%;
    margin-left: 3%;
  }

  .content {
    background-color: var(--neutral-100);
    box-shadow: 0px 0px 5px 1px var(--neutral-300);
    border: 1px solid var(--neutral-300);
    width: 35vw;
    height: fit-content;
    max-height: 75%;
    margin: auto;
    padding: 25px;
  }
</style>
