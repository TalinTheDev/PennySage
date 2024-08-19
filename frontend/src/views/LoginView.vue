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

  const errors = ref(new Map());
  const validate = (target, name) => {
    if (!target.checkValidity()) {
      errors.value.set(`${name}Err`, target.validationMessage);
    } else {
      errors.value.delete(`${name}Err`);
    }
  };
  provide("errors", errors);
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
        userStore.login(data.user_details, data.access_token);
        router.push({ name: "dashboard" });
      })
      .catch((error) => {
        alert(`Error: ${JSON.stringify(error)}`);
      });
  };
</script>

<template>
  <AuthLayout>
    <h2>Login <RouterLink to="register" class="registerLink">Or Register</RouterLink></h2>
    <div class="form">
      <InputBox name="Username">
        <input
          type="text"
          required
          placeholder="Username"
          v-model="username"
          @blur="validate($event.target, 'Username')" />
      </InputBox>
      <InputBox name="Password">
        <input
          type="password"
          required
          placeholder="Password"
          @keydown.enter="handleLogin"
          v-model="password"
          @blur="validate($event.target, 'Password')" />
      </InputBox>
      <button @click="handleLogin">Login</button>
    </div>
  </AuthLayout>
</template>

<style scoped>
  .form {
    width: 100%;
    text-align: center;
  }

  input {
    background-color: transparent;
    border: 0;
    width: 100%;
    border-bottom: 2px solid var(--neutral-500);
    padding: 10px 0px;

    &:focus {
      outline: none;
    }
    &:has(+ .errorMsg:is(.invalidLabel)) {
      border-color: var(--red-600);
    }
  }
  button {
    background-color: inherit;
    font-size: 1.2rem;
    width: 100%;
    padding: 10px;
    margin-top: 10px;
    border: 1px solid var(--neutral-500);

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
