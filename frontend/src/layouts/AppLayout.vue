<script setup>
  import { ref } from "vue";
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";
  import NavDropdown from "@/components/NavDropdown.vue";
  import IconField from "primevue/iconfield";
  import InputIcon from "primevue/inputicon";
  import InputText from "primevue/inputtext";

  const user = useUserStore();
  const router = useRouter();
  const page = ref(router.currentRoute.value.name);
</script>
<template>
  <div class="page">
    <nav>
      <div class="top">
        <div class="logo">
          <span>Penny Sage</span> <i class="pi pi-sign-out" @click="user.logout()"></i>
        </div>
        <IconField>
          <InputIcon class="pi pi-search" />
          <InputText v-model="search" placeholder="Search" unstyled />
        </IconField>
      </div>
      <div class="links">
        <NavDropdown title="Home">
          <RouterLink to="/">
            <div class="navLink" :class="{ activeLink: page === 'dashboard' }">
              <i class="pi pi-home"></i>
              Dashboard
            </div>
          </RouterLink>
        </NavDropdown>
        <NavDropdown title="Transactions">
          <RouterLink to="/add" class="">
            <div class="navLink" :class="{ activeLink: page === 'add' }">
              <i class="pi pi-plus"></i>
              Add
            </div>
          </RouterLink>
          <RouterLink to="/view">
            <div class="navLink" :class="{ activeLink: page === 'view' }">
              <i class="pi pi-table"></i>
              View
            </div>
          </RouterLink>
        </NavDropdown>
        <NavDropdown title="Insights">
          <RouterLink to="/analytics">
            <div class="navLink" :class="{ activeLink: page === 'analytics' }">
              <i class="pi pi-chart-bar"></i>
              Analytics
            </div>
          </RouterLink>
        </NavDropdown>
      </div>
      <div class="bottomActions">
        <RouterLink to="/settings">
          <div class="navLink" :class="{ activeLink: page === 'settings' }">
            <i class="pi pi-cog"></i>
          </div>
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
    grid-template-columns: clamp(100px, 20%, 350px) 80%;
    grid-template-areas: "nav content";
  }

  i.pi {
    /* Icons */
    margin-right: 0.5rem;
  }

  nav {
    grid-area: nav;
    display: grid;
    grid-template-rows: min-content 1fr min-content;
    grid-template-areas:
      "top"
      "links"
      "bottomActions";
    padding: 1.25rem;

    .top {
      grid-area: top;
      margin-bottom: 1rem;

      .p-inputicon {
        color: var(--neutral-950);
      }

      input {
        /* Search */
        width: 100%;
        border-radius: 10px;
        margin: 15px 0;
        padding: clamp(5px, 0.75rem, 10px) clamp(10px, 1rem, 15px) clamp(5px, 0.75rem, 10px) 2.25rem;
        border: 0px;
        background-color: var(--neutral-200);
        color: var(--neutral-950);
      }
      .logo {
        display: flex;
        align-items: center;
        justify-content: space-between;

        span {
          /* Logo */
          font-size: 1.2rem;
          font-weight: bold;
        }

        i {
          margin-right: 0;
          cursor: pointer;

          &:hover {
            color: var(--neutral-500);
          }
        }
      }
    }
    .links {
      grid-area: links;
    }
    .bottomActions {
      grid-area: bottomActions;
      display: flex;
      align-items: end;

      i {
        margin-right: 0;
      }
    }

    .links,
    .bottomActions {
      a {
        display: block;
        font-weight: bold;
        text-decoration: none;
        color: var(--neutral-500);
        margin: 5px 0px;

        .navLink {
          margin: 0 auto;
          padding: 5px 10px;
          border-radius: 10px;

          &:hover {
            background-color: var(--neutral-50);
            box-shadow: 0px 2px 2px 0px var(--neutral-400);
            color: var(--neutral-950);
          }
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
