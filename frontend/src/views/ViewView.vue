<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import Button from "@/components/Button.vue";
  import DataTable from "primevue/datatable";
  import Column from "primevue/column";
  import InputText from "primevue/inputtext";
  import InputNumber from "primevue/inputnumber";
  import PageHeader from "@/components/PageHeader.vue";
  import IconField from "primevue/iconfield";
  import InputIcon from "primevue/inputicon";
  import DatePicker from "primevue/datepicker";

  import { FilterMatchMode } from "@primevue/core/api";
  import { ref, onBeforeMount, onMounted, computed } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useToast } from "primevue/usetoast";
  import { useConfirm } from "primevue/useconfirm";

  const toast = useToast();
  const confirm = useConfirm();
  const api = useApiStore().api;

  const transactions = ref([]);
  const mappedTransactions = ref(new Map());

  const transactionsCount = computed(() => {
    return transactions.value.length;
  });
  const selectedItems = ref();
  const rows = ref(5);
  const first = ref(0);
  const editingRows = ref([]);

  const selectAll = ref(false);
  const loading = ref(false);

  const filters = ref();

  const initFilters = () => {
    filters.value = {
      global: { value: null, matchMode: FilterMatchMode.CONTAINS },
      desc: { value: null, matchMode: FilterMatchMode.CONTAINS },
      date: { value: null, matchMode: FilterMatchMode.DATE_IS },
      amount: { value: null, matchMode: FilterMatchMode.CONTAINS },
      merchant: { value: null, matchMode: FilterMatchMode.CONTAINS },
      category: { value: null, matchMode: FilterMatchMode.CONTAINS },
      payment_source: { value: null, matchMode: FilterMatchMode.CONTAINS }
    };
  };

  const getTransactions = () => {
    loading.value = true;
    api
      .get("/transactions/all")
      .then(({ data }) => {
        data.transactions.forEach((transaction) => {
          transaction.date = new Date(transaction.date);
          mappedTransactions.value.set(transaction.id, transaction);
        });
        transactions.value = Array.from(mappedTransactions.value.values());
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Transactions Get Failed",
          detail: error.message,
          life: 3000
        });
      });
    loading.value = false;
  };

  const deleteTransactions = () => {
    if (selectedItems.value.length === 0) {
      return;
    }
    confirm.require({
      message: "Do you want to delete these transactions?",
      header: "Delete Transactions",
      icon: "pi pi-info-circle",
      rejectProps: {
        label: "Cancel",
        severity: "secondary",
        outlined: true
      },
      acceptProps: {
        label: "Delete",
        severity: "danger"
      },
      accept: () => {
        let transactionsToBeDeleted = [];
        selectedItems.value.forEach((transaction) => {
          transactionsToBeDeleted.push(transaction.id);
        });
        loading.value = true;
        api
          .delete("/transactions/delete", {
            data: {
              transactions: transactionsToBeDeleted
            }
          })
          .then((data) => {
            toast.add({
              severity: "info",
              summary: "Transaction Deleted",
              detail: data,
              life: 3000
            });
            transactionsToBeDeleted.forEach((id) => {
              mappedTransactions.value.delete(id);
            });
            transactions.value = Array.from(mappedTransactions.value.values());
            selectedItems.value = [];
            loading.value = false;
          })
          .catch((error) => {
            toast.add({
              severity: "error",
              summary: "Transactions Delete Failed",
              detail: error.message,
              life: 3000
            });
            loading.value = false;
          });
      },
      reject: () => {}
    });
  };

  const compareTransactions = (oldTransaction, newTransaction) => {
    let diff = [];
    if (oldTransaction.desc !== newTransaction.desc) {
      diff.push({ field: "desc", value: newTransaction.desc });
    }
    if (oldTransaction.date !== newTransaction.date) {
      diff.push({ field: "date", value: new Date(newTransaction.date).toISOString() });
    }
    if (oldTransaction.amount !== newTransaction.amount) {
      diff.push({ field: "amount", value: newTransaction.amount.toString() });
    }
    if (oldTransaction.merchant !== newTransaction.merchant) {
      diff.push({ field: "merchant", value: newTransaction.merchant });
    }
    if (oldTransaction.category !== newTransaction.category) {
      diff.push({ field: "category", value: newTransaction.category });
    }
    if (oldTransaction.payment_source !== newTransaction.payment_source) {
      diff.push({ field: "payment_source", value: newTransaction.payment_source });
    }

    return diff;
  };

  const editTransaction = (event) => {
    let { newData, data } = event;
    mappedTransactions.value.set(data.id, newData);
    transactions.value = Array.from(mappedTransactions.value.values());
    let edits = compareTransactions(data, newData);
    loading.value = true;
    api
      .patch("/transactions/edit", {
        id: newData.id,
        edits: edits
      })
      .then((data) => {
        toast.add({
          severity: "info",
          summary: "Transaction Edited",
          detail: data,
          life: 3000
        });

        getTransactions();
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Transaction Edit Failed",
          detail: error.message,
          life: 3000
        });
        mappedTransactions.value.set(data.id, data);
        transactions.value = Array.from(mappedTransactions.value.values());
        loading.value = false;
      });
  };

  const onSelectAllChange = (event) => {
    selectAll.value = event.checked;
    if (selectAll.value) {
      selectedItems.value = transactions.value;
    } else {
      selectedItems.value = [];
    }
  };

  const onRowUnselect = () => {
    selectAll.value = false;
  };

  const onRowSelect = () => {
    selectAll.value = selectedItems.value.length === transactions.value.length;
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
    <PageHeader>
      <template #header>
        <span>View Transactions</span>
      </template>
    </PageHeader>
    <div class="tableHeader">
      <div class="search">
        <IconField>
          <InputIcon class="pi pi-search" />
          <InputText
            v-model="filters['global'].value"
            placeholder="Search"
            pt:root:class="searchInput" />
        </IconField>
      </div>
      <div class="actions">
        <Button @click="initFilters">Clear Filters</Button>
        <Button @click="deleteTransactions">Delete Selected Transactions</Button>
      </div>
      <p class="totalTransactions">{{ transactionsCount }} transactions found</p>
      <p class="selectedTransactions">
        {{ selectedItems ? selectedItems.length : 0 }} transactions selected
      </p>
    </div>

    <DataTable
      v-model:value="transactions"
      dataKey="id"
      v-model:loading="loading"
      rowHover
      sortMode="multiple"
      removableSort
      size="small"
      v-model:filters="filters"
      filterDisplay="menu"
      :globalFilterFields="['desc', 'amount', 'merchant', 'category', 'payment_source']"
      v-model:first="first"
      v-model:rows="rows"
      paginator
      :rowsPerPageOptions="[5, 10, 20, 50]"
      v-model:selection="selectedItems"
      :selectAll="selectAll"
      @select-all-change="onSelectAllChange"
      @row-select="onRowSelect"
      @row-unselect="onRowUnselect"
      @value-change="onValueChange"
      v-model:editingRows="editingRows"
      editMode="row"
      @row-edit-save="editTransaction">
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
        <template #body="{ data }"
          >{{ parseFloat(data.amount).toFixed(2) > 0 ? "" : "-" }}${{
            Math.abs(parseFloat(data.amount)).toFixed(2)
          }}
        </template>
        <template #editor="{ data, field }">
          <InputNumber mode="currency" currency="USD" v-model="data[field]" fluid />
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
  .tableHeader {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-template-rows: 1fr 1fr;
    grid-template-areas:
      "search actions"
      "totalTransactions selectedTransactions";
    column-gap: 0.5rem;
    align-items: center;
    padding: 1rem;

    .search {
      grid-area: search;

      .searchInput {
        width: 100%;
      }
    }

    .actions {
      grid-area: actions;
      display: flex;
      column-gap: 0.5rem;

      button {
        margin: 0;
      }
    }
    p {
      font-size: 1.05rem;
    }
    .totalTransactions {
      grid-area: totalTransactions;
    }
    .selectedTransactions {
      grid-area: selectedTransactions;
      text-align: right;
    }
  }
</style>
