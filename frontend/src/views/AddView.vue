<script setup>
  import { ref, provide, watchEffect } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";
  import InputBox from "@/components/InputBox.vue";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useRouter } from "vue-router";

  const router = useRouter();
  const api = useApiStore().api;
  const description = ref("");
  const dateInput = ref(new Date().toISOString().slice(0, 10));
  let date = 0;
  const amount = ref("");
  const transactionType = ref("debit");
  const merchant = ref("");
  const category = ref("");
  const paymentSource = ref("");
  // const items = ref([]);

  const errors = ref(new Map());
  const validateAmount = (target) => {
    fixAmount(target);
    validate(target, "Amount");
  };
  const fixAmount = (target) => {
    amount.value = parseFloat(amount.value).toFixed(2);
    target.value = parseFloat(amount.value).toFixed(2);
  };
  const validate = (target, name) => {
    if (!target.checkValidity()) {
      errors.value.set(`${name}Err`, target.validationMessage);
    } else {
      errors.value.delete(`${name}Err`);
    }
  };
  provide("errors", errors);

  watchEffect(() => {
    date = new Date(dateInput.value.split("-")).setHours(0, 0, 0, 0);
  });

  const handleAddTransaction = () => {
    if (errors.value.size !== 0) {
      return;
    }

    api
      .post("/transactions/new", {
        desc: description.value,
        date: date,
        amount: amount.value.toString(),
        type: transactionType.value,
        merchant: merchant.value,
        category: category.value,
        payment_source: paymentSource.value
      })
      .then((data) => {
        alert(`Transaction Added: ID ${data.transaction_id}`);
        router.push({ name: "view" });
      })
      .catch((error) => {
        alert(`Error: ${JSON.stringify(error)}`);
      });
  };
</script>
<template>
  <AppLayout>
    <h1>Add Transaction</h1>
    <div class="form">
      <InputBox name="Description">
        <input
          type="text"
          required
          placeholder="Description"
          v-model="description"
          @blur="validate($event.target, 'Description')" />
      </InputBox>
      <div class="columns">
        <InputBox name="Date">
          <input
            type="date"
            required
            placeholder="Date"
            v-model="dateInput"
            @blur="validate($event.target, 'Date')" />
        </InputBox>

        <InputBox name="Amount">
          <input
            type="number"
            required
            min="0"
            step="0.01"
            placeholder="Amount"
            v-model="amount"
            @blur="validateAmount($event.target)" />
        </InputBox>
        <InputBox name="Type">
          <select
            required
            placeholder="Transaction Type"
            v-model="transactionType"
            @blur="validate($event.target, 'Type')">
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
            @blur="validate($event.target, 'Merchant')" />
        </InputBox>
        <InputBox name="Category">
          <input
            type="text"
            required
            placeholder="Category"
            v-model="category"
            @blur="validate($event.target, 'Category')" />
        </InputBox>
        <InputBox name="Payment Source">
          <input
            type="text"
            required
            placeholder="Payment Source"
            v-model="paymentSource"
            @blur="validate($event.target, 'Payment Source')"
            @keydown.enter="handleAddTransaction" />
        </InputBox>
      </div>

      <button @click="handleAddTransaction">Add Transaction</button>
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
</style>
