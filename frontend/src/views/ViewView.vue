<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  // import DebitChip from "@/components/DebitChip.vue";
  // import CreditChip from "@/components/CreditChip.vue";
  import DataTable from "primevue/datatable";
  import Column from "primevue/column";
  import InputText from "primevue/InputText";
  import DatePicker from "primevue/datepicker";

  import Select from "primevue/select";

  import Button from "primevue/button";
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
    <h2>View Transactions</h2>
    <DataTable
      :value="transactions"
      tableStyle="min-width: 50rem"
      showGridlines
      sortMode="multiple"
      removableSort
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
        <InputText v-model="filters['global'].value" placeholder="Keyword Search" />
        <Button type="button" label="Clear Filters" outlined @click="initFilters()" />
        <Button
          @click="deleteTransactions"
          label="Delete Selected Transactions"
          severity="danger"
          outlined />
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

    <!-- <table class="table" v-if="transactions.length > 0">
      <thead>
        <tr class="searchBoxes">
          <th></th>
          <th>
            <input
              type="text"
              @input="descriptionSearchFilter"
              v-model="descriptionSearch"
              placeholder="Description Search" />
          </th>
          <th>
            <input
              type="date"
              @input="dateSearchFilter"
              v-model="dateSearch"
              placeholder="Date Search" />
          </th>
          <th></th>
          <th></th>
          <th>
            <input
              type="text"
              @input="merchantSearchFilter"
              v-model="merchantSearch"
              placeholder="Merchant Search" />
          </th>
          <th>
            <input
              type="text"
              @input="categorySearchFilter"
              v-model="categorySearch"
              placeholder="Category Search" />
          </th>
          <th>
            <input
              type="text"
              @input="sourceSearchFilter"
              v-model="sourceSearch"
              placeholder="Source Search" />
          </th>
        </tr>
        <tr>
          <th><input type="checkbox" @click="checkAll" v-model="allSelected" /></th>
          <th
            @click="
              () => {
                if (sortDesc === 0) {
                  sortDesc = 1;
                } else if (sortDesc === 1) {
                  sortDesc = -1;
                } else {
                  sortDesc = 0;
                }
              }
            ">
            Description {{ sortDesc }}
          </th>
          <th
            @click="
              () => {
                if (sortDate === 0) {
                  sortDate = 1;
                } else if (sortDate === 1) {
                  sortDate = -1;
                } else {
                  sortDate = 0;
                }
              }
            ">
            Date {{ sortDate }}
          </th>
          <th>Amount</th>
          <th>Payment Type</th>
          <th>Merchant</th>
          <th>Category</th>
          <th>Payment Source</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="transaction in sortedTransactions" :key="transaction.transaction.id">
          <td>
            <input
              type="checkbox"
              v-model="checkedTransactions"
              :value="transaction.transaction.id" />
          </td>
          <td>{{ transaction.transaction.desc }}</td>
          <td>{{ new Date(transaction.transaction.date).toLocaleDateString() }}</td>
          <td>${{ parseFloat(transaction.transaction.amount).toFixed(2) }}</td>
          <td><DebitChip v-if="transaction.transaction.type == 'debit'" /><CreditChip v-else /></td>
          <td>{{ transaction.transaction.merchant }}</td>
          <td>{{ transaction.transaction.category }}</td>
          <td>{{ transaction.transaction.payment_source }}</td>
        </tr>
      </tbody>
    </table> -->
  </AppLayout>
</template>

<style scoped>
  /* .table {
    border: 1px solid var(--black);
    border-collapse: collapse;
    width: 100%;
    margin-top: 1%;

    th,
    td {
      border: 1px solid var(--black);
      padding: 5px 10px;
      text-align: center;
      display: table-cell;
      vertical-align: middle;
    }

    .searchBoxes th {
      border: none;
    }
  }
  input:not([type="checkbox"]) {
    background-color: transparent;
    border: 0;
    width: 85%;
    font-size: 0.8rem;
    text-align: center;
    border-bottom: 2px solid var(--dark-grey);
    padding: 5px 5px;

    &:focus {
      outline: none;
    }
  }
  button {
    background-color: inherit;
    font-size: 0.8rem;
    width: 20%;
    padding: 5px;
    margin: 10px 0px;
    border: 1px solid var(--black);

    &:hover {
      cursor: pointer;
    }
  } */
</style>
