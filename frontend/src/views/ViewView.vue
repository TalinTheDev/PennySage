<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import DataTable from "primevue/datatable";
  import Column from "primevue/column";
  import InputText from "primevue/inputtext";
  import DatePicker from "primevue/datepicker";

  import Select from "primevue/select";
  import Tag from "primevue/tag";

  import { FilterMatchMode } from "@primevue/core/api";
  import { ref, onBeforeMount, onMounted } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";

  const transactions = ref([]);
  const api = useApiStore().api;
  const selectedTransactions = ref();
  const typeFilterOptions = ref([
    { label: "Debit", value: "debit" },
    { label: "Credit", value: "credit" }
  ]);
  const filters = ref();

  const initFilters = () => {
    filters.value = {
      global: { value: null, matchMode: FilterMatchMode.CONTAINS },
      "transaction.desc": { value: null, matchMode: FilterMatchMode.CONTAINS },
      "transaction.date": { value: null, matchMode: FilterMatchMode.DATE_IS },
      "transaction.amount": { value: null, matchMode: FilterMatchMode.CONTAINS },
      "transaction.type": { value: null, matchMode: FilterMatchMode.EQUALS },
      "transaction.merchant": { value: null, matchMode: FilterMatchMode.CONTAINS },
      "transaction.category": { value: null, matchMode: FilterMatchMode.CONTAINS },
      "transaction.payment_source": { value: null, matchMode: FilterMatchMode.CONTAINS }
    };
  };

  const getTransactions = () => {
    api
      .get("/transactions/all")
      .then((data) => {
        data.forEach((transaction) => {
          transaction.transaction.date = new Date(transaction.transaction.date);
          transaction.transaction.amount = `$${parseFloat(transaction.transaction.amount).toFixed(2)}`;
        });
        transactions.value = data;
      })
      .catch((error) => {
        alert(`Error: ${JSON.stringify(error.message)}`);
      });
  };

  const deleteTransactions = () => {
    if (selectedTransactions.value.length === 0) {
      return;
    }
    let transactions = [];
    selectedTransactions.value.forEach((transaction) => {
      transactions.push(transaction.transaction.id);
    });
    api
      .delete("/transactions/delete", {
        data: {
          transactions: transactions
        }
      })
      .then((data) => {
        alert(data);
        getTransactions();
      })
      .catch((error) => {
        alert(`Error: ${JSON.stringify(error.message)}`);
      });
  };

  onBeforeMount(() => {
    initFilters();
  });
  onMounted(() => {
    getTransactions();
  });
</script>
<template>
  <AppLayout>
    <DataTable
      :value="transactions"
      tableStyle="min-width: 50rem"
      showGridlines
      sortMode="multiple"
      removableSort
      :multiSortMeta="[{ field: 'transaction.date', order: -1 }]"
      size="small"
      v-model:filters="filters"
      dataKey="transaction.id"
      filterDisplay="menu"
      :globalFilterFields="[
        'transaction.desc',
        'transaction.amount',
        'transaction.merchant',
        'transaction.category',
        'transaction.payment_source'
      ]"
      v-model:selection="selectedTransactions">
      <template #header>
        <h2>View Transactions</h2>
        <InputText v-model="filters['global'].value" placeholder="Keyword Search" />
        <button @click="initFilters">Clear Filters</button>
        <button @click="deleteTransactions">Delete Selected Transactions</button>
      </template>
      <template #empty> No transactions found. </template>
      <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
      <Column field="transaction.id" hidden />
      <Column field="transaction.desc" header="Description" sortable>
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by description" />
        </template>
      </Column>
      <Column field="transaction.date" header="Date" sortable dataType="date">
        <template #body="{ data }">
          {{ new Date(data.transaction.date).toLocaleDateString() }}
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <DatePicker v-model="filterModel.value" @date-select="filterCallback()" />
        </template>
      </Column>
      <Column field="transaction.amount" sortable header="Amount">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by amount" />
        </template>
      </Column>
      <Column field="transaction.type" header="Type" sortable>
        <template #body="{ data }">
          <Tag severity="danger" value="Debit" v-if="data.transaction.type === 'debit'"></Tag>
          <Tag severity="success" value="Credit" v-if="data.transaction.type === 'credit'"></Tag>
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <Select
            v-model="filterModel.value"
            optionLabel="label"
            optionValue="value"
            :options="typeFilterOptions"
            placeholder="Filter by Type"
            @change="filterCallback()" />
        </template>
      </Column>
      <Column field="transaction.merchant" sortable header="Merchant">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by merchant" />
        </template>
      </Column>
      <Column field="transaction.category" sortable header="Category">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by category" />
        </template>
      </Column>
      <Column field="transaction.payment_source" sortable header="Payment Source">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by payment source" />
        </template>
      </Column>
    </DataTable>
  </AppLayout>
</template>

<style scoped>
  button {
    background-color: inherit;
    font-size: 0.9rem;
    width: 20%;
    padding: 5px;
    margin: 10px 10px;
    border: 1px solid var(--black);

    &:hover {
      cursor: pointer;
    }
  }
</style>
