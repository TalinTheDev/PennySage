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
  import AutoComplete from "primevue/autocomplete";
  import DatePicker from "primevue/datepicker";
  import InputNumber from "primevue/inputnumber";
  import FileUpload from "primevue/fileupload";
  import { useToast } from "primevue/usetoast";

  const toast = useToast();
  const router = useRouter();
  const api = useApiStore().api;
  const description = ref("");
  const dateInput = ref(new Date());
  let date = "0000-00-00T00:00:00.000Z";
  const amount = ref(0.0);
  const merchant = ref("");
  const merchantOptionsAll = ref([]);
  const merchantOptions = ref([]);
  const categoryOptionsAll = ref([]);
  const categoryOptions = ref([]);
  const sourceOptionsAll = ref([]);
  const sourceOptions = ref([]);
  const category = ref("");
  const paymentSource = ref("");
  const newMOption = ref("");
  const newCOption = ref("");
  const newSOption = ref("");

  const fileupload = ref();

  const errors = ref(new Map());
  const validateDate = (value) => {
    if (value == null) {
      errors.value.set("DateErr", "Please enter a valid date");
    } else {
      errors.value.delete("DateErr");
    }
  };
  const validateAmount = (value) => {
    if (parseFloat(value.substring(1)) === 0) {
      errors.value.set("AmountErr", "Amount cannot be equal to $0.00");
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
    date = new Date(dateInput.value).toISOString();
  });

  const checkEmptyValues = () => {
    if (description.value == "") {
      errors.value.set("DescriptionErr", "Please enter a description");
    } else {
      errors.value.delete("DescriptionErr");
    }
    if (date == "0000-00-00T00:00:00.000Z" || dateInput.value == null) {
      errors.value.set("DateErr", "Please enter a valid date");
    } else {
      errors.value.delete("DateErr");
    }
    if (parseFloat(amount.value) === 0 || amount.value === null) {
      errors.value.set("AmountErr", "Amount must not be equal to $0.00");
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
        merchant: merchant.value.value ? merchant.value.value : merchant.value,
        category: category.value.value ? category.value.value : category.value,
        payment_source: paymentSource.value.value ? paymentSource.value.value : paymentSource.value
      })
      .then(({ data }) => {
        toast.add({
          severity: "info",
          summary: "Transaction Added",
          detail: `ID: ${data.transaction_id}`,
          life: 3000
        });
        router.push({ name: "view" });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Error adding transaction",
          detail: error.message,
          life: 3000
        });
      });
  };
  const handleNewMOption = () => {
    api
      .post("/options/new", { value: newMOption.value, type: "M" })
      .then(({ data }) => {
        toast.add({
          severity: "info",
          summary: "Merchant Option Added",
          detail: `ID: ${data.option_id}`,
          life: 3000
        });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Error adding merchant option",
          detail: error.message,
          life: 3000
        });
      });
  };
  const handleNewCOption = () => {
    api
      .post("/options/new", { value: newCOption.value, type: "C" })
      .then(({ data }) => {
        toast.add({
          severity: "info",
          summary: "Category Option Added",
          detail: `ID: ${data.option_id}`,
          life: 3000
        });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Error adding category option",
          detail: error.message,
          life: 3000
        });
      });
  };
  const handleNewSOption = () => {
    api
      .post("/options/new", { value: newSOption.value, type: "S" })
      .then(({ data }) => {
        toast.add({
          severity: "info",
          summary: "Source Option Added",
          detail: `ID: ${data.option_id}`,
          life: 3000
        });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Error adding source option",
          detail: error.message,
          life: 3000
        });
      });
  };
  const handleImportTransactions = () => {
    const config = { headers: { "Content-Type": "multipart/form-data" } };
    const form = new FormData();
    form.append("file", fileupload.value.files[0]);
    api
      .post("/transactions/import", form, config)
      .then((data) => {
        toast.add({
          severity: "info",
          summary: data.message,
          detail: `Count: ${data.transaction_count}`,
          life: 3000
        });
        router.push({ name: "view" });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Error importing transaction",
          detail: error.message,
          life: 3000
        });
      });
  };

  const search = (e, t) => {
    switch (t) {
      case "m":
        merchantOptions.value = merchantOptionsAll.value.filter(
          (option) => option.value.toLowerCase().search(e.query.toLowerCase()) > -1
        );
        break;
      case "c":
        categoryOptions.value = categoryOptionsAll.value.filter(
          (option) => option.value.toLowerCase().search(e.query.toLowerCase()) > -1
        );
        break;
      case "s":
        sourceOptions.value = sourceOptionsAll.value.filter(
          (option) => option.value.toLowerCase().search(e.query.toLowerCase()) > -1
        );
        break;
    }
  };
  api.get("/options/get").then(({ data }) => {
    merchantOptionsAll.value = data.options.filter((option) => option.type == "M");
    categoryOptionsAll.value = data.options.filter((option) => option.type == "C");
    sourceOptionsAll.value = data.options.filter((option) => option.type == "S");
  });
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
        </div>
        <div class="columns">
          <InputBox name="Merchant">
            <AutoComplete
              fluid
              id="merchant"
              v-model="merchant"
              placeholder="Merchant"
              required
              :suggestions="merchantOptions"
              optionLabel="value"
              @complete="
                (event) => {
                  search(event, 'm');
                }
              "
              @blur="
                (event) => {
                  validate(event.target, 'Merchant');
                }
              " />
          </InputBox>
          <InputBox name="Category">
            <AutoComplete
              fluid
              id="category"
              v-model="category"
              placeholder="Category"
              required
              optionLabel="value"
              :suggestions="categoryOptions"
              @complete="
                (event) => {
                  search(event, 'c');
                }
              "
              @blur="
                (event) => {
                  validate(event.target, 'Category');
                }
              " />
          </InputBox>
          <InputBox name="Payment Source">
            <AutoComplete
              fluid
              id="paymentSource"
              optionLabel="value"
              v-model="paymentSource"
              placeholder="Payment Source"
              required
              :suggestions="sourceOptions"
              @complete="
                (event) => {
                  search(event, 's');
                }
              "
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
    <Card>
      <template #header>
        <span class="formHeader">Import Transactions</span>
      </template>
      <template #content>
        <div class="importBox">
          <div class="fileInput">
            <FileUpload
              ref="fileupload"
              mode="basic"
              name="file[]"
              accept=".csv"
              :maxFileSize="1000000"
              @upload="handleImportTransactions"
              pt:root:class="fileInput" />
          </div>
          <div class="inputSubmit">
            <Button @click="handleImportTransactions" pt:root:class="fileUploadButton">
              Import Transactions
            </Button>
          </div>
        </div>
      </template>
    </Card>
    <Card>
      <template #header>
        <span class="formHeader">Add Options</span>
      </template>
      <template #content>
        <div class="columns">
          <InputBox name="newMOption">
            <InputText
              fluid
              id="newMOption"
              v-model="newMOption"
              placeholder="New Merchant Option"
              required
              @blur="
                (event) => {
                  validate(event.target, 'MerchantOption');
                }
              " />
          </InputBox>
          <Button @click="handleNewMOption">Add Merchant</Button>
        </div>
        <div class="columns">
          <InputBox name="newCOption">
            <InputText
              fluid
              id="newCOption"
              v-model="newCOption"
              placeholder="New Category Option"
              required
              @blur="
                (event) => {
                  validate(event.target, 'CategoryOption');
                }
              " />
          </InputBox>
          <Button @click="handleNewCOption">Add Category</Button>
        </div>
        <div class="columns">
          <InputBox name="newSOption">
            <InputText
              fluid
              id="newSOption"
              v-model="newSOption"
              placeholder="New Source Option"
              required
              @blur="
                (event) => {
                  validate(event.target, 'SourceOption');
                }
              " />
          </InputBox>
          <Button @click="handleNewSOption">Add Source</Button>
        </div>
      </template>
    </Card>
  </AppLayout>
</template>
<style scoped>
  .columns {
    display: flex;
    column-gap: 1rem;
  }
  .importBox {
    display: grid;
    grid-template-columns: 2fr 1fr;
    column-gap: 0.5rem;
    grid-template-areas: "fileInput inputSubmit";
    align-items: center;
  }
  .fileInput {
    grid-area: fileInput;
  }
  .inputSubmit {
    grid-area: inputSubmit;
  }
  #merchant *,
  #category *,
  #paymentSource * {
    width: 100%;
  }
</style>
