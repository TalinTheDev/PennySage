<script setup>
  import { ref, provide, watchEffect } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";
  import InputBox from "@/components/InputBox.vue";
  import Card from "@/components/Card.vue";
  import Button from "@/components/Button.vue";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useRouter } from "vue-router";
  import InputText from "primevue/inputtext";
  import DatePicker from "primevue/datepicker";
  import InputNumber from "primevue/inputnumber";
  import SelectButton from "primevue/selectbutton";

  const router = useRouter();
  const api = useApiStore().api;
  const description = ref("");
  const dateInput = ref(new Date());
  let date = 0;
  const amount = ref(0.0);
  const transactionType = ref("debit");
  const merchant = ref("");
  const category = ref("");
  const paymentSource = ref("");
  // const items = ref([]);
  const typeOptions = ref([
    { name: "Debit", value: "debit" },
    { name: "Credit", value: "credit" }
  ]);

  const errors = ref(new Map());
  const validateDate = (value) => {
    if (value == null) {
      errors.value.set("DateErr", "Please enter a valid date");
    } else {
      errors.value.delete("DateErr");
    }
  };
  const validateAmount = (value) => {
    if (!(parseFloat(value.substring(1)) > 0)) {
      errors.value.set("AmountErr", "Amount must be greater than $0.00");
    } else {
      errors.value.delete("AmountErr");
    }
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

  const checkEmptyValues = () => {
    if (description.value == "") {
      errors.value.set("DescriptionErr", "Please enter a description");
    } else {
      errors.value.delete("DescriptionErr");
    }
    if (date < 0 || dateInput.value == null) {
      errors.value.set("DateErr", "Please enter a valid date");
    } else {
      errors.value.delete("DateErr");
    }
    if (!(parseFloat(amount.value) > 0) || amount.value == null) {
      errors.value.set("AmountErr", "Amount must be greater than $0.00");
    } else {
      errors.value.delete("AmountErr");
    }
    if (merchant.value == "") {
      errors.value.set("MerchantErr", "Please enter a Merchant");
    } else {
      errors.value.delete("MerchantErr");
    }
    if (category.value == "") {
      errors.value.set("CategoryErr", "Please enter a category");
    } else {
      errors.value.delete("CategoryErr");
    }
    if (paymentSource.value == "") {
      errors.value.set("Payment SourceErr", "Please enter a payment source");
    } else {
      errors.value.delete("Payment SourceErr");
    }
  };

  const handleAddTransaction = () => {
    checkEmptyValues();
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
      <template #header>
        <span class="formHeader">Transaction Details</span>
      </template>
      <template #content>
        <InputBox name="Description">
          <InputText
            fluid
            id="description"
            v-model="description"
            placeholder="Description"
            required
            @blur="
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
              @update:model-value="validateDate"
              showButtonBar
              showIcon
              iconDisplay="input" />
          </InputBox>

          <InputBox name="Amount">
            <InputNumber
              v-model="amount"
              mode="currency"
              currency="USD"
              locale="en-US"
              fluid
              showButtons
              @blur="
                ({ value }) => {
                  validateAmount(value);
                }
              " />
          </InputBox>
          <InputBox name="Type">
            <SelectButton
              v-model="transactionType"
              optionLabel="name"
              optionValue="value"
              pt:root:class="typeButton"
              @blur="
                (event) => {
                  validate(event.target, 'Type');
                }
              "
              :options="typeOptions" />
          </InputBox>
        </div>
        <div class="columns">
          <InputBox name="Merchant">
            <InputText
              fluid
              id="merchant"
              v-model="merchant"
              placeholder="Merchant"
              required
              @blur="
                (event) => {
                  validate(event.target, 'Merchant');
                }
              " />
          </InputBox>
          <InputBox name="Category">
            <InputText
              fluid
              id="category"
              v-model="category"
              placeholder="Category"
              required
              @blur="
                (event) => {
                  validate(event.target, 'Category');
                }
              " />
          </InputBox>
          <InputBox name="Payment Source">
            <InputText
              fluid
              id="paymentSource"
              v-model="paymentSource"
              placeholder="Payment Source"
              required
              @blur="
                (event) => {
                  validate(event.target, 'Payment Source');
                }
              " />
          </InputBox>
        </div>
        <Button @click="handleAddTransaction">Add Transaction</Button>
      </template>
    </Card>
  </AppLayout>
</template>
<style scoped>
  .columns {
    display: flex;
    column-gap: 1rem;
  }
</style>
