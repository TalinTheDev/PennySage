<script setup>
  import { ref, provide } from "vue";
  import { RouterLink } from "vue-router";
  import InputBox from "@/components/InputBox.vue";
  import AuthLayout from "../layouts/AuthLayout.vue";

  const handleRegister = () => {
    if (errors.value.size !== 0) {
      return;
    }
    alert("Register");
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
      <InputBox type="text" placeholder="Username" />
      <InputBox type="email" placeholder="Email" />
      <div class="double-column">
        <InputBox type="text" placeholder="First Name" />
        <InputBox type="text" placeholder="Last Name" />
      </div>
      <div class="double-column">
        <InputBox type="text" placeholder="Password" />
        <InputBox type="text" placeholder="Confirm Password" @keydown.enter="handleRegister" />
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
