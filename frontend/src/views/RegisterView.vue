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

  const errors = ref(new Map());
  const validate = (target, name) => {
    if (!target.checkValidity()) {
      errors.value.set(`${name}Err`, target.validationMessage);
    } else {
      errors.value.delete(`${name}Err`);
    }
  };
  provide("errors", errors);

  const handleRegister = () => {
    if (password.value !== confirmPassword.value) {
      errors.value.set("Confirm PasswordErr", "Passwords do not match");
    }
    if (errors.value.size !== 0) {
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
</script>

<template>
  <AuthLayout>
    <h2>Register <RouterLink to="login" class="loginLink">Or Login</RouterLink></h2>
    <div class="form">
      <InputBox name="Username">
        <input
          type="text"
          required
          placeholder="Username"
          v-model="username"
          @blur="validate($event.target, 'Username')" />
      </InputBox>
      <InputBox name="Email">
        <input
          type="email"
          required
          placeholder="Email"
          v-model="email"
          @blur="validate($event.target, 'Email')" />
      </InputBox>
      <div class="columns">
        <InputBox name="First Name">
          <input
            type="text"
            required
            placeholder="First Name"
            v-model="firstName"
            @blur="validate($event.target, 'First Name')" />
        </InputBox>
        <InputBox name="Last Name">
          <input
            type="text"
            required
            placeholder="Last Name"
            v-model="lastName"
            @blur="validate($event.target, 'Last Name')" />
        </InputBox>
      </div>
      <div class="columns">
        <InputBox name="Password">
          <input
            type="password"
            required
            placeholder="Password"
            v-model="password"
            @blur="validate($event.target, 'Password')" />
        </InputBox>
        <InputBox name="Confirm Password">
          <input
            type="password"
            required
            placeholder="Confirm Password"
            @keydown.enter="handleLogin"
            v-model="confirmPassword"
            @blur="validate($event.target, 'Confirm Password')" />
        </InputBox>
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

  .columns {
    display: flex;
    flex-direction: row;
    column-gap: 2%;
  }
  input {
    background-color: transparent;
    border: 0;
    width: 100%;
    border-bottom: 2px solid var(--dark-grey);
    padding: 10px 0px;

    &:focus {
      outline: none;
    }
    &:has(+ .errorMsg:is(.invalidLabel)) {
      border-color: red;
    }
  }
  button {
    background-color: inherit;
    font-size: 1.2rem;
    width: 100%;
    padding: 10px;
    margin-top: 10px;
    border: 1px solid var(--black);

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
