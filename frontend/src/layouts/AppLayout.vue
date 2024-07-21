<script setup>
  import Button from "primevue/button";
  import Toolbar from "primevue/toolbar";
  import { ref } from "vue";
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";

  const user = useUserStore();
  const router = useRouter();
  const page = ref(router.currentRoute.value.name);
</script>
<template>
  <div class="page">
    <div class="header">
      <h1>PennySage</h1>
      <nav>
        <div class="leftLinks">
          <RouterLink
            to="/"
            class="navLink"
            :class="{ activeLink: page === 'dashboard' }"
            @click="changeActivePage('dashboard')">
            Dashboard
          </RouterLink>
          <RouterLink
            to="/add"
            class="navLink"
            :class="{ activeLink: page === 'add' }"
            @click="changeActivePage('add')"
            >Add Transaction</RouterLink
          >
          <RouterLink
            to="/view"
            class="navLink"
            :class="{ activeLink: page === 'view' }"
            @click="changeActivePage('view')"
            >View Transactions</RouterLink
          >
        </div>
        <div class="rightLinks">
          <RouterLink
            to="/settings"
            class="navLink"
            :class="{ activeLink: page === 'settings' }"
            @click="changeActivePage('settings')"
            >Settings</RouterLink
          >
          <a to="javascript:void(0)" class="navLink button" @click="user.logout()">Logout</a>
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
    display: flex;
    flex-direction: row;
    align-items: center;

    nav {
      display: flex;
      flex-grow: 1;
    }

    .nav {
      flex-grow: 1;
      border: 0;
      padding: 0% 6% 0% 3%;
      width: max-content;
    }

    .navLink {
      margin: 0% 2%;
      min-width: fit-content;
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
      padding: 3% 6%;

      &:hover,
      &:focus {
        background-color: var(--black);
        cursor: pointer;
        color: var(--white);
      }
    }
  }

  .content {
    background-color: var(--white);
    width: 100%;
    flex-grow: 1;
    padding: 1% 3%;
  }
</style>
