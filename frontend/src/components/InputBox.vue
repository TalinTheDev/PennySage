<script setup>
  import { ref, inject } from "vue";

  const props = defineProps(["type", "placeholder"]);
  const model = defineModel();

  const errorMsg = ref("");
  const { addError, removeError } = inject("errors");

  const setErrorMsg = (err) => {
    errorMsg.value = err;
  };
  const emailRegex = new RegExp(
    // eslint-disable-next-line no-useless-escape
    "[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?"
  );
  const validate = () => {
    if (!model.value) {
      setErrorMsg(`${props.placeholder} must not be empty`);
      addError(props.placeholder);
      return;
    }
    if (props.type === "email" && !emailRegex.test(model.value)) {
      setErrorMsg("Invalid email format");
      addError(props.placeholder);
    } else {
      removeError(props.placeholder);
      setErrorMsg("");
    }
  };
</script>

<template>
  <div class="inputField">
    <input :type="type" :placeholder="props.placeholder" v-model="model" @blur="validate" />
    <span class="errorMsg" :class="{ invalidLabel: errorMsg }">{{ errorMsg }}&nbsp;</span>
  </div>
</template>

<style scoped>
  .inputField {
    margin: 1% 0;
    width: 100%;
    display: block;

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

    .errorMsg {
      color: red;
      font-size: 0.8rem;
      display: block;
      text-align: start;

      &:not(.invalidLabel) {
        color: transparent;
      }
    }
  }
</style>
