<script setup>
  import { ref, provide } from "vue";
  import { RouterLink } from "vue-router";
  import InputBox from "@/components/InputBox.vue";
  import Card from "@/components/Card.vue";
  import Button from "@/components/Button.vue";
  import InputText from "primevue/inputtext";
  import Password from "primevue/password";
  import AuthLayout from "../layouts/AuthLayout.vue";
  import { useApiStore } from "@/stores/useApiStore";
  import { useUserStore } from "@/stores/useUserStore";
  import { useRouter } from "vue-router";
  import { useToast } from "primevue/usetoast";

  const toast = useToast();
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
        toast.add({
          severity: "error",
          summary: "Error logging in",
          detail: error.message,
          life: 3000
        });
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
        <Button @click="handleLogin">Login</Button>
      </template>
    </Card>
  </AuthLayout>
</template>

<style scoped>
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
