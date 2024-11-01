<script setup>
  import { ref } from "vue";
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";
  import NavDropdown from "@/components/NavDropdown.vue";

  const user = useUserStore();
  const router = useRouter();
  const page = ref(router.currentRoute.value.name);

  const showUserActions = ref(false);
</script>
<template>
  <div class="page">
    <nav>
      <div class="top">
        <div class="logo">
          <span>Penny Sage</span>
        </div>
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
        <div class="userDetails">
          <span class="username">{{ user.firstName }}</span>
          <span class="userEmail">{{ user.email }}</span>
        </div>
        <div class="userActions">
          <div class="ellipses" @click="showUserActions = !showUserActions">
            <div class="ellipse" />
            <div class="ellipse" />
            <div class="ellipse" />
          </div>
          <div class="userActionsContent" v-show="showUserActions">
            <div class="userAction">
              <RouterLink to="/settings">
                <div>
                  <i class="pi pi-cog"></i>
                  Settings
                </div>
              </RouterLink>
            </div>
            <div class="userAction">
              <a href="#" @click="user.logout()">
                <i class="pi pi-sign-out"></i>
                Logout
              </a>
            </div>
          </div>
        </div>
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
    grid-template-columns: clamp(100px, 20%, 325px) 1fr;
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
    background-color: var(--white);
    border-right: 3px solid var(--border);
    box-shadow: 3px 0px 3px 0px var(--border);

    .top {
      padding: 1.25rem 1.25rem 0;
      grid-area: top;
      margin-bottom: 1rem;

      .logo {
        margin-bottom: 0.2rem;

        span {
          /* Logo */
          font-size: 1.5rem;
          font-weight: bold;
        }
      }
    }
    .links {
      padding: 0 1.25rem 1.25rem;
      grid-area: links;
    }
    .bottomActions {
      padding: 1.25rem;
      grid-area: bottomActions;
      display: flex;
      align-items: center;
      justify-content: space-between;

      .userDetails {
        display: flex;
        flex-direction: column;

        .username {
          font-weight: bold;
          font-size: 1rem;
        }
        .userEmail {
          font-size: 0.8rem;
          color: var(--light-grey);
        }
      }
      .userActions {
        .ellipses {
          padding: 1rem;
          display: flex;
          column-gap: 0.1rem;

          .ellipse {
            width: 0.3rem;
            height: 0.3rem;
            border-radius: 100%;
            background-color: var(--black);
          }
        }

        .userActionsContent {
          background-color: var(--white);
          border: 1px solid var(--border);
          padding: 1.2rem;
          position: fixed;
          bottom: 4rem;
          left: clamp(250px, 15rem, 400px);
          display: flex;
          flex-direction: column;
          row-gap: 0.8rem;
        }
      }
    }

    .links,
    .bottomActions {
      a {
        display: block;
        font-weight: 650;
        text-decoration: none;
        color: var(--light-grey);
        margin: 5px 0px;

        .navLink {
          margin: 0 auto;
          padding: 5px 10px;

          &:hover:not(.activeLink) {
            box-shadow: 0px 2px 2px 0px var(--border);
            color: var(--accent-hover);
            border-left: 5px solid var(--accent-hover);
          }
        }
      }

      .activeLink {
        box-shadow: 0px 2px 2px 0px var(--border);
        color: var(--accent-highlight);
        border-left: 5px solid var(--accent-highlight);
      }
    }
  }

  .content {
    grid-area: content;
    overflow-y: scroll;
  }
</style>
