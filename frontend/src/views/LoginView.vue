<script setup>
  const loginHandler = () => {};
</script>

<template>
  <div class="container w-screen h-screen flex items-center justify-center">
    <FormKit
      type="form"
      submit-label="Login"
      @submit="loginHandler"
      :actions="false">
      <h1 class="text-6xl font-bold mb-2 flex justify-center">Login</h1>
      <FormKit
        type="text"
        name="login"
        label="Your email"
        placeholder="john@example.com"
        validation="required|email" />
      <FormKit
        type="password"
        name="password"
        label="Password"
        validation="required|length:6|matches:/[^a-zA-Z]/"
        :validation-messages="{
          matches: 'Please include at least one symbol'
        }"
        placeholder="Your password" />
      <FormKit type="submit" label="Login" />
    </FormKit>
  </div>
</template>
