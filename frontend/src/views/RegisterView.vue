// TODO: Add family option

<script setup>
  import { ref, provide } from "vue";
  import { RouterLink } from "vue-router";
  import InputBox from "@/components/InputBox.vue";
  import AuthLayout from "../layouts/AuthLayout.vue";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";

  const api = useApiStore().api;
  const userStore = useUserStore();
  const router = useRouter();

  const username = ref("");
  const email = ref("");
  const firstName = ref("");
  const lastName = ref("");
  const password = ref("");
  const confirmPassword = ref("");

  const handleRegister = () => {
    if (errors.value.size !== 0 || password.value !== confirmPassword.value) {
      return;
    }
    api
      .post("/auth/register", {
        username: username.value,
        email: email.value,
        first_name: firstName.value,
        last_name: lastName.value,
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
    <h2>Register <RouterLink to="login" class="loginLink">Or Login</RouterLink></h2>
    <div class="form">
      <InputBox type="text" placeholder="Username" v-model="username" />
      <InputBox type="email" placeholder="Email" v-model="email" />
      <div class="double-column">
        <InputBox type="text" placeholder="First Name" v-model="firstName" />
        <InputBox type="text" placeholder="Last Name" v-model="lastName" />
      </div>
      <div class="double-column">
        <InputBox type="text" placeholder="Password" v-model="password" />
        <InputBox
          type="text"
          placeholder="Confirm Password"
          v-model="confirmPassword"
          @keydown.enter="handleRegister" />
      </div>
      <button @click="handleRegister">Register</button>
    </div>
  </AuthLayout>
</template>

<style scoped>
  .form {
    width: 100%;
    text-align: center;
  }

  .double-column {
    display: flex;
    flex-direction: row;
    column-gap: 2%;
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

  .loginLink {
    font-size: 0.8rem;
    color: inherit;
    text-underline-offset: 10%;
  }
</style>
