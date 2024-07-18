<script setup>
  import Button from "primevue/button";

  import { RouterLink } from "vue-router";
  import { ref } from "vue";
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";

  const user = useUserStore();
  const router = useRouter();
  const page = ref(router.currentRoute.value.name);

  const changeActivePage = (pageName) => {
    page.value = pageName;
  };
</script>
<template>
  <div class="page">
    <div class="header">
      <h1>PennySage</h1>
      <nav>
        <div class="leftLinks">
          <Button
            as="router-link"
            label="Dashboard"
            to="/"
            class="navLink"
            :class="{ activeLink: page === 'dashboard' }"
            link />
          <Button
            as="router-link"
            label="Add Transaction"
            to="/add"
            class="navLink"
            :class="{ activeLink: page === 'add' }"
            link />
          <Button
            as="router-link"
            label="View Transactions"
            to="/view"
            class="navLink"
            :class="{ activeLink: page === 'view' }"
            link />
        </div>
        <div class="rightLinks">
          <Button
            as="router-link"
            label="Settings"
            to="/settings"
            class="navLink"
            :class="{ activeLink: page === 'settings' }"
            link />
          <Button label="Logout" class="navLink button" unstyled @click="user.logout()" />
        </div>
      </nav>
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
    flex-grow: 0;
    flex-shrink: 1;
    display: flex;
    align-items: baseline;

    nav {
      padding: 0% 2%;
      flex-grow: 1;
      display: flex;
      justify-content: space-between;
    }

    .leftLinks,
    .rightLinks {
      display: flex;
      flex-grow: 1;

      .navLink {
        margin: 0% 2%;
        color: initial;
        text-decoration: none;
        text-underline-offset: 10%;

        &:hover,
        &:focus {
          text-decoration: underline;
        }
      }

      .navLink.activeLink {
        text-decoration: underline;
      }

      .navLink.button {
        border: 1px solid var(--black);
        background-color: transparent;
        padding: 0 2%;

        &:hover,
        &:focus {
          background-color: var(--black);
          color: var(--white);
        }
      }
    }

    .rightLinks {
      justify-content: flex-end;
    }
  }

  .content {
    background-color: var(--white);
    width: 100%;
    flex-grow: 1;
    padding: 1% 3%;
  }
</style>
