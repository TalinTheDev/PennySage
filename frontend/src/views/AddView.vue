<script setup>
  import { ref, provide } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";
  import InputBox from "@/components/InputBox.vue";

  const description = ref("");
  const date = ref("");
  const amount = ref("");
  const transactionType = ref("debit");
  const merchant = ref("");
  const category = ref("");
  const paymentSource = ref("");
  const items = ref([]);

  const errors = ref(new Map());

  const validateText = (target, name) => {
    if (!target.checkValidity()) {
      errors.value.set(`${name}Err`, target.validationMessage);
    } else {
      errors.value.delete(`${name}Err`);
    }
  };
  provide("errors", errors);
</script>
<template>
  <AppLayout>
    <h2>Add Transaction</h2>
    <div class="form">
      <InputBox name="Description">
        <input
          type="text"
          required
          placeholder="Description"
          v-model="description"
          @blur="validateText($event.target, 'Description')" />
      </InputBox>
      <div class="columns">
        <InputBox name="Date">
          <input
            type="date"
            required
            placeholder="Date"
            v-model="date"
            @blur="validateText($event.target, 'Date')" />
        </InputBox>

        <InputBox name="Amount">
          <input
            type="number"
            required
            min="0"
            step="0.01"
            placeholder="Amount"
            v-model="amount"
            @blur="validateText($event.target, 'Amount')" />
        </InputBox>
        <InputBox name="Type">
          <select
            required
            placeholder="Transaction Type"
            v-model="transactionType"
            @blur="validateText($event.target, 'Type')">
            <option value="placeholder" disabled hidden>Select Transaction Type</option>
            <option value="debit">Debit</option>
            <option value="credit">Credit</option>
          </select>
        </InputBox>
      </div>

      <div class="columns">
        <InputBox name="Merchant">
          <input
            type="text"
            required
            placeholder="Merchant"
            v-model="merchant"
            @blur="validateText($event.target, 'Merchant')" />
        </InputBox>
        <InputBox name="Category">
          <input
            type="text"
            required
            placeholder="Category"
            v-model="category"
            @blur="validateText($event.target, 'Category')" />
        </InputBox>
        <InputBox name="Payment Source">
          <input
            type="text"
            required
            placeholder="Payment Source"
            v-model="paymentSource"
            @blur="validateText($event.target, 'Payment Source')" />
        </InputBox>
      </div>
    </div>
  </AppLayout>
</template>

<style scoped>
  .columns {
    display: flex;
    flex-direction: row;
    column-gap: 2%;
  }

  input,
  select {
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
</style>
