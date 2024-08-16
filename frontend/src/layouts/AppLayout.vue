<script setup>
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
          <RouterLink to="/" class="navLink" :class="{ activeLink: page === 'dashboard' }">
            Dashboard
          </RouterLink>
          <RouterLink to="/add" class="navLink" :class="{ activeLink: page === 'add' }">
            Add Transaction
          </RouterLink>
          <RouterLink to="/view" class="navLink" :class="{ activeLink: page === 'view' }">
            View Transactions
          </RouterLink>
        </div>
        <div class="rightLinks">
          <RouterLink to="/settings" class="navLink" :class="{ activeLink: page === 'settings' }">
            Settings
          </RouterLink>
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
    margin: 2rem 2rem 0 2rem;
    flex-grow: 0;
    flex-shrink: 1;
    display: flex;
    align-items: baseline;

    nav {
      padding: 0% 1.5rem;
      flex-grow: 1;
      display: flex;
      justify-content: space-between;
    }

    .leftLinks,
    .rightLinks {
      display: flex;
      align-items: center;

      flex-grow: 1;

      .navLink {
        margin: 0% 1.5rem;
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
        padding: 0.5rem 1.5rem;
        margin-left: 0;
        margin-right: 0px;

        &:hover,
        &:focus {
          background-color: var(--dark-grey);
          border-color: var(--dark-grey);
          color: var(--white);
          cursor: pointer;
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
