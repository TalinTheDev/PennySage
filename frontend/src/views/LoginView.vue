<script setup>
  import { ref, provide } from "vue";
  import { RouterLink } from "vue-router";
  import InputBox from "@/components/InputBox.vue";
  import Card from "@/components/Card.vue";
  import InputText from "primevue/inputtext";
  import Password from "primevue/password";
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

  const checkEmptyValues = () => {
    if (username.value == "") {
      errors.value.set("UsernameErr", "Please enter your username");
    } else {
      errors.value.delete("UsernameErr");
    }
    if (password.value == "") {
      errors.value.set("PasswordErr", "Please enter your password");
    } else {
      errors.value.delete("PasswordErr");
    }
  };
  const handleLogin = () => {
    checkEmptyValues();
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
    <Card>
      <template #header>
        <h1>Login <RouterLink to="register" class="registerLink">Or Register</RouterLink></h1>
      </template>
      <template #content>
        <InputBox name="Username">
          <InputText
            fluid
            v-model="username"
            placeholder="Username"
            required
            @blur="
              (event) => {
                validate(event.target, 'Username');
              }
            " />
        </InputBox>
        <InputBox name="Password">
          <Password
            fluid
            v-model="password"
            :feedback="false"
            required
            placeholder="Password"
            @keydown.enter="handleLogin"
            @blur="
              (event) => {
                validate(event.target, 'Password');
              }
            " />
        </InputBox>
        <button @click="handleLogin">Login</button>
      </template>
    </Card>
  </AuthLayout>
</template>
<style>
  .p-inputtext {
    outline: none;
    border: 1px solid var(--neutral-500);
    background-color: transparent;
  }
  :is(.p-inputtext, .p-inputwrapper):has(+ .invalidLabel) {
    border-radius: var(--p-form-field-border-radius);
    border: 2px solid var(--red-600);
  }
  .p-inputwrapper:has(+ .invalidLabel) .p-password-input {
    border: none;
  }
</style>

<style scoped>
  button {
    background-color: var(--teal-600);
    color: var(--white);
    width: 100%;
    padding: 0.5rem;
    margin-top: 10px;
    border: none;

    &:hover {
      background-color: var(--teal-700);
      cursor: pointer;
    }
  }

  .registerLink {
    font-size: 0.75rem;
    color: inherit;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
      text-underline-offset: 50%;
    }
  }
</style>
