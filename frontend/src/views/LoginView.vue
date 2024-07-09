<script setup>
  const handleLogin = () => {};
</script>

<template>
  <div class="page">
    <div class="header">
      <h1>PennySage</h1>
    </div>
    <div class="container">
      <h2>Login</h2>
      <div class="form">
        <input type="text" placeholder="Email or Username" /> <br />
        <input type="password" placeholder="Password" /> <br />
        <button @click="handleLogin">Login</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .page {
    width: 100%;
    height: 100%;
    display: flex;
  }
  .header {
    position: absolute;
    top: 3%;
    left: 1%;
  }

  .container {
    background-color: var(--white);
    box-shadow: 1px 1px 10px 1px var(--black);
    border-radius: 12px;
    width: 25%;
    height: 60%;
    margin: auto;
    padding: 5px;
    display: flex;
    flex-direction: column;
    align-items: center;

    h2 {
      margin-bottom: 25px;
      margin-top: 15px;
    }

    .form {
      width: 90%;
      text-align: center;
    }

    input {
      margin: 5px 0px;
      background-color: transparent;
      border: 1px solid var(--black);
      border-radius: 4px;
      padding: 15px 10px;
      width: 100%;

      &:focus {
        outline: none;
      }
    }

    button {
      margin-top: 10px;
      width: 100%;
      padding: 15px 10px;
      background-color: var(--primary);
      border: none;
      border-radius: 50px;

      &:hover {
        cursor: pointer;
      }
    }
  }
</style>
