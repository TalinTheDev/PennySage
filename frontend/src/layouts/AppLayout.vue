<script setup>
  import { ref } from "vue";
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";
  import NavDropdown from "@/components/NavDropdown.vue";

  const user = useUserStore();
  const router = useRouter();
  const page = ref(router.currentRoute.value.name);
</script>
<template>
  <div class="page">
    <nav>
      <div class="top">
        <a>Penny Sage</a> <span @click="user.logout()">Logout</span>
        <input placeholder="Search" />
      </div>
      <div class="links">
        <RouterLink to="/">
          <div class="navLink" :class="{ activeLink: page === 'dashboard' }">Dashboard</div>
        </RouterLink>
        <NavDropdown
          title="Transactions"
          :active="page === 'add' || page === 'view'"
          class="navGroup">
          <RouterLink to="/add" class="">
            <div class="navLink" :class="{ activeLink: page === 'add' }">Add</div>
          </RouterLink>
          <RouterLink to="/view">
            <div class="navLink" :class="{ activeLink: page === 'view' }">View</div>
          </RouterLink>
        </NavDropdown>
        <RouterLink to="/analytics">
          <div class="navLink" :class="{ activeLink: page === 'analytics' }">Analytics</div>
        </RouterLink>
      </div>
      <div class="bottomActions">
        <RouterLink to="/settings">
          <div class="navLink" :class="{ activeLink: page === 'settings' }">Settings</div>
        </RouterLink>
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
    display: grid;
    grid-template-columns: clamp(100px, 20%, 400px) 80%;
    grid-template-areas: "nav content";
  }

  nav {
    grid-area: nav;
    display: grid;
    grid-template-rows: 20% 70% 10%;
    grid-template-areas:
      "top"
      "links"
      "bottomActions";
    padding: 10% 10% 0% 10%;

    .top {
      grid-area: top;

      input {
        width: 100%;
        border-radius: 10px;
        padding: 10px 15px;
        margin: 15px 0;
        border: 0px;
        background-color: var(--neutral-200);
      }

      a {
        font-size: 1.2rem;
        font-weight: bold;
      }
    }
    .links {
      grid-area: links;

      .navGroup > *:not(.navGroupHeader):not(.groupActive) {
        display: none;
      }
    }
    .bottomActions {
      grid-area: bottomActions;
    }

    .links,
    .bottomActions {
      a,
      span {
        display: block;
        font-weight: bold;
        text-decoration: none;
        color: var(--neutral-500);
        margin: 5px 0px;

        .navLink {
          margin: 0 auto;
          padding: 5px 10px;
          border-radius: 10px;
        }
      }

      .activeLink {
        background-color: var(--neutral-50);
        box-shadow: 0px 2px 2px 0px var(--neutral-400);
        color: var(--neutral-950);
      }
    }
  }

  .content {
    grid-area: content;
    padding: 2rem 2rem 2rem;
  }
</style>
