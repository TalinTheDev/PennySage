<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import DataTable from "primevue/datatable";
  import Column from "primevue/column";
  import InputText from "primevue/inputtext";
  import InputNumber from "primevue/inputnumber";
  import DatePicker from "primevue/datepicker";

  import Select from "primevue/select";
  import Tag from "primevue/tag";

  import { FilterMatchMode } from "@primevue/core/api";
  import { ref, onBeforeMount, onMounted } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";

  const transactions = ref();
  const api = useApiStore().api;
  const selectedTransactions = ref();
  const typeFilterOptions = ref([
    { label: "Debit", value: "debit" },
    { label: "Credit", value: "credit" }
  ]);
  const filters = ref();
  const editingRows = ref([]);

  const initFilters = () => {
    filters.value = {
      global: { value: null, matchMode: FilterMatchMode.CONTAINS },
      desc: { value: null, matchMode: FilterMatchMode.CONTAINS },
      date: { value: null, matchMode: FilterMatchMode.DATE_IS },
      amount: { value: null, matchMode: FilterMatchMode.CONTAINS },
      type: { value: null, matchMode: FilterMatchMode.EQUALS },
      merchant: { value: null, matchMode: FilterMatchMode.CONTAINS },
      category: { value: null, matchMode: FilterMatchMode.CONTAINS },
      payment_source: { value: null, matchMode: FilterMatchMode.CONTAINS }
    };
  };

  const getTransactions = () => {
    api
      .get("/transactions/all")
      .then((data) => {
        data.forEach((transaction) => {
          transaction.date = new Date(transaction.date);
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
      transactions.push(transaction.id);
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

  const editTransaction = (event) => {
    let { newData, index } = event;
    let oldData = transactions.value[index];
    transactions.value[index] = newData;
    let edits = Object.fromEntries(Object.entries(newData).filter(([k, v]) => oldData[k] !== v));
    edits = Object.keys(edits).map((key) => [{ field: key, value: edits[key].toString() }]);
    edits.forEach((edit, index) => {
      edits[index] = edit[0];
      if (edits[index].field == "desc") edits[index].field = "description";
    });
    api
      .patch("/transactions/edit", {
        id: newData.id,
        edits: edits
      })
      .then((data) => {
        alert(data);
      })
      .catch((error) => {
        alert(`Error: ${JSON.stringify(error)}`);
        transactions.value[index] = oldData;
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
      showGridlines
      sortMode="multiple"
      removableSort
      :multiSortMeta="[{ field: 'date', order: -1 }]"
      size="small"
      v-model:filters="filters"
      dataKey="id"
      filterDisplay="menu"
      :globalFilterFields="['desc', 'amount', 'merchant', 'category', 'payment_source']"
      v-model:selection="selectedTransactions"
      v-model:editingRows="editingRows"
      editMode="row"
      @row-edit-save="editTransaction">
      <template #header>
        <h2>View Transactions</h2>
        <InputText v-model="filters['global'].value" placeholder="Keyword Search" />
        <button @click="initFilters">Clear Filters</button>
        <button @click="deleteTransactions">Delete Selected Transactions</button>
      </template>
      <template #empty> No transactions found. </template>
      <Column selectionMode="multiple"></Column>
      <Column field="id" hidden />
      <Column field="desc" header="Description" sortable>
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by description" />
        </template>
        <template #editor="{ data, field }">
          <InputText v-model="data[field]" fluid />
        </template>
      </Column>
      <Column field="date" header="Date" sortable dataType="date">
        <template #body="{ data }">
          {{ new Date(data.date).toLocaleDateString() }}
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <DatePicker v-model="filterModel.value" @date-select="filterCallback()" />
        </template>
        <template #editor="{ data, field }">
          <DatePicker v-model="data[field]" fluid />
        </template>
      </Column>
      <Column field="amount" sortable header="Amount">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by amount" />
        </template>
        <template #body="{ data }"> ${{ parseFloat(data.amount).toFixed(2) }} </template>
        <template #editor="{ data, field }">
          <InputNumber mode="currency" currency="USD" v-model="data[field]" fluid />
        </template>
      </Column>
      <Column field="type" header="Type" sortable>
        <template #body="{ data }">
          <Tag severity="danger" value="Debit" v-if="data.type === 'debit'"></Tag>
          <Tag severity="success" value="Credit" v-if="data.type === 'credit'"></Tag>
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
        <template #editor="{ data, field }">
          <Select
            v-model="data[field]"
            fluid
            :options="typeFilterOptions"
            optionLabel="label"
            optionValue="value" />
        </template>
      </Column>
      <Column field="merchant" sortable header="Merchant">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by merchant" />
        </template>
        <template #editor="{ data, field }">
          <InputText v-model="data[field]" fluid />
        </template>
      </Column>
      <Column field="category" sortable header="Category">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by category" />
        </template>
        <template #editor="{ data, field }">
          <InputText v-model="data[field]" fluid />
        </template>
      </Column>
      <Column field="payment_source" sortable header="Payment Source">
        <template #filter="{ filterModel, filterCallback }">
          <InputText
            v-model="filterModel.value"
            type="text"
            @input="filterCallback()"
            placeholder="Filter by payment source" />
        </template>
        <template #editor="{ data, field }">
          <InputText v-model="data[field]" fluid />
        </template>
      </Column>
      <Column :rowEditor="true" bodyStyle="text-align:center"></Column>
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
