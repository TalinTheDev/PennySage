<script setup>
  import { ref, provide, watchEffect } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";
  import InputBox from "@/components/InputBox.vue";
  import Card from "@/components/Card.vue";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useRouter } from "vue-router";
  import InputText from "primevue/inputtext";
  import DatePicker from "primevue/datepicker";

  const router = useRouter();
  const api = useApiStore().api;
  const description = ref("");
  const dateInput = ref();
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
    date = new Date(dateInput.value).setHours(0, 0, 0, 0);
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
    <PageHeader>
      <template #header>
        <span>Add Transaction</span>
      </template>
    </PageHeader>
    <Card>
      <InputBox name="Description">
        <InputText
          id="description"
          v-model="description"
          unstyled
          placeholder="Description"
          required
          :pt:root:onBlur="
            (event) => {
              validate(event.target, 'Description');
            }
          " />
      </InputBox>
      <div class="columns">
        <InputBox name="Date">
          <DatePicker
            required
            fluid
            v-model="dateInput"
            dateFormat="mm/dd/yy"
            :pt:root:onBlur="
              (event) => {
                validate(event.target, 'Date');
              }
            "
            showButtonBar
            showIcon
            iconDisplay="input" />
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
          <InputText
            id="merchant"
            v-model="merchant"
            unstyled
            placeholder="Merchant"
            required
            :pt:root:onBlur="
              (event) => {
                validate(event.target, 'Merchant');
              }
            " />
        </InputBox>
        <InputBox name="Category">
          <InputText
            id="category"
            v-model="category"
            unstyled
            placeholder="Category"
            required
            :pt:root:onBlur="
              (event) => {
                validate(event.target, 'Category');
              }
            " />
        </InputBox>
        <InputBox name="Payment Source">
          <InputText
            id="paymentSource"
            v-model="paymentSource"
            unstyled
            placeholder="Payment Source"
            required
            :pt:root:onBlur="
              (event) => {
                validate(event.target, 'Payment Source');
              }
            " />
        </InputBox>
      </div>

      <button @click="handleAddTransaction">Add Transaction</button>
    </Card>
  </AppLayout>
</template>

<style scoped>
  .columns {
    display: flex;
    flex-direction: row;
    column-gap: 2%;
  }
  input,
  select,
  .input {
    background-color: transparent;
    width: 100%;
    border: 2px solid var(--neutral-500);
    padding: 10px;

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
