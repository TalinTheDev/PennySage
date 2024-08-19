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
    <nav>
      <div class="logo">
        <a class="">PennySage</a>
      </div>
      <div class="links">
        <RouterLink to="/" class="navLink" :class="{ activeLink: page === 'dashboard' }">
          Dashboard
        </RouterLink>
        <RouterLink to="/add" class="navLink" :class="{ activeLink: page === 'add' }">
          New
        </RouterLink>
        <RouterLink to="/view" class="navLink" :class="{ activeLink: page === 'view' }">
          Transactions
        </RouterLink>
      </div>
      <div class="misc">
        <RouterLink to="/settings" class="navLink" :class="{ activeLink: page === 'settings' }">
          ⚙️
        </RouterLink>
        <a to="javascript:void(0)" class="navLink button" @click="user.logout()">👋</a>
      </div>
    </nav>
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

  nav {
    padding: 1.5rem;
    display: grid;
    grid-template-columns: 1fr 2fr 1fr;
    grid-template-areas: "logo links misc";
  }

  .logo {
    grid-area: logo;
    display: flex;
    align-items: center;
    a {
      font-size: 1.5rem;
      font-weight: bold;
    }
  }

  .links {
    grid-area: links;
    display: flex;
    align-items: center;
    justify-content: space-evenly;

    a {
      font-size: 1rem;
    }
  }

  .misc {
    grid-area: misc;
    display: flex;
    align-items: center;
    justify-content: end;

    a {
      margin: 0px 5px;
    }
  }

  .navLink {
    color: var(--dark-grey);
    font-weight: bold;
    text-decoration: none;
    text-underline-offset: 10%;
    cursor: pointer;
    padding: 10px 20px;
    border-radius: 4px;
  }

  .navLink.activeLink {
    background-color: CornflowerBlue;
    color: var(--white);
  }

  .navLink.button {
  }
  .content {
    background-color: var(--white);
    width: 100%;
    flex-grow: 1;
    padding: 0 1.5rem;
  }
</style>
