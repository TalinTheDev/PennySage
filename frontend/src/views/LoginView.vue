<script setup>
  import { ref, provide } from "vue";
  import { RouterLink } from "vue-router";
  import InputBox from "@/components/InputBox.vue";
  import AuthLayout from "../layouts/AuthLayout.vue";
  import { useApiStore } from "@/stores/useApiStore";
  import { useUserStore } from "@/stores/useUserStore";
  import { useRouter } from "vue-router";

  const api = useApiStore().api;
  const userStore = useUserStore();
  const router = useRouter();

  const username = ref("");
  const password = ref("");

  const handleLogin = () => {
    if (errors.value.size !== 0) {
      return;
    }
    api
      .post("/auth/login", {
        username: username.value,
        password: password.value
      })
      .then((data) => {
        userStore.login(data.data.user_details, data.data.access_token);
        router.push({ name: "dashboard" });
      })
      .catch((error) => {
        alert(`Error: ${error.response.data.message}`);
      });
  };

  const errors = ref(new Set());
  const addError = (field) => {
    errors.value.add(field);
  };
  const removeError = (field) => {
    errors.value.delete(field);
  };
  provide("errors", { addError, removeError });
</script>

<template>
  <AuthLayout>
    <h2>Login <RouterLink to="register" class="registerLink">Or Register</RouterLink></h2>
    <div class="form">
      <InputBox type="text" placeholder="Username" v-model="username" />
      <InputBox
        type="password"
        placeholder="Password"
        @keydown.enter="handleLogin"
        v-model="password" />
      <button @click="handleLogin">Login</button>
    </div>
  </AuthLayout>
</template>

<style scoped>
  .form {
    width: 100%;
    text-align: center;
  }

  button {
    background-color: inherit;
    font-size: 1.2rem;
    width: 100%;
    padding: 10px;
    margin-top: 10px;
    border: 1px solid var(--black);
    border-radius: 50px;

    &:hover {
      cursor: pointer;
    }
  }

  .registerLink {
    font-size: 0.8rem;
    color: inherit;
    text-underline-offset: 10%;
  }
</style>
