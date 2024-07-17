<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import DebitChip from "@/components/DebitChip.vue";
  import CreditChip from "@/components/CreditChip.vue";
  import { ref, onMounted, computed } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";

  const transactions = ref([]);
  const api = useApiStore().api;
  const checkedTransactions = ref([]);
  const allSelected = ref(false);
  const descriptionSearch = ref("");
  const dateSearch = ref();
  const merchantSearch = ref("");
  const categorySearch = ref("");
  const sourceSearch = ref("");

  const getTransactions = () => {
    api
      .get("/transactions/all")
      .then((data) => {
        transactions.value = data;
      })
      .catch((error) => {
        alert(`Error: ${JSON.stringify(error.message)}`);
      });
  };
  const checkAll = () => {
    if (!allSelected.value) {
      transactions.value.map((transaction) => {
        checkedTransactions.value.push(transaction.transaction.id);
      });
    } else {
      checkedTransactions.value = [];
      allSelected.value = false;
    }
  };

  const deleteTransactions = () => {
    if (checkedTransactions.value.length === 0) {
      return;
    }
    api
      .delete("/transactions/delete", {
        data: {
          transactions: checkedTransactions.value
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

  const descriptionSearchFilter = () => {
    if (descriptionSearch.value === "") {
      return transactions.value;
    } else {
      return transactions.value.filter((transaction) => {
        return transaction.transaction.desc
          .toString()
          .toLowerCase()
          .includes(descriptionSearch.value);
      });
    }
  };

  const dateSearchFilter = () => {
    if (!dateSearch.value) {
      return transactions.value;
    } else {
      return transactions.value.filter((transaction) => {
        let searchDate = new Date(dateSearch.value.split("-")).setHours(0, 0, 0, 0);
        return transaction.transaction.date == searchDate;
      });
    }
  };

  const merchantSearchFilter = () => {
    if (merchantSearch.value === "") {
      return transactions.value;
    } else {
      return transactions.value.filter((transaction) => {
        return transaction.transaction.merchant
          .toString()
          .toLowerCase()
          .includes(merchantSearch.value);
      });
    }
  };

  const categorySearchFilter = () => {
    if (categorySearch.value === "") {
      return transactions.value;
    } else {
      return transactions.value.filter((transaction) => {
        return transaction.transaction.category
          .toString()
          .toLowerCase()
          .includes(categorySearch.value);
      });
    }
  };

  const sourceSearchFilter = () => {
    if (sourceSearch.value === "") {
      return transactions.value;
    } else {
      return transactions.value.filter((transaction) => {
        return transaction.transaction.payment_source
          .toString()
          .toLowerCase()
          .includes(sourceSearch.value);
      });
    }
  };

  const filteredTransactions = computed(() => {
    return Array.from(
      new Set(descriptionSearchFilter())
        .intersection(new Set(dateSearchFilter()))
        .intersection(new Set(merchantSearchFilter()))
        .intersection(new Set(categorySearchFilter()))
        .intersection(new Set(sourceSearchFilter()))
    );
  });

  onMounted(getTransactions);
</script>
<template>
  <AppLayout>
    <h2>View Transactions</h2>
    <p v-if="transactions.length === 0">No Transactions Found</p>
    <button @click="deleteTransactions" v-if="transactions.length > 0">
      Delete Selected Transactions
    </button>

    <table class="table" v-if="transactions.length > 0">
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
          <th>Description</th>
          <th>Date</th>
          <th>Amount</th>
          <th>Payment Type</th>
          <th>Merchant</th>
          <th>Category</th>
          <th>Payment Source</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="transaction in filteredTransactions" :key="transaction.transaction.id">
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
    </table>
  </AppLayout>
</template>

<style scoped>
  .table {
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
    margin-top: 10px;
    border: 1px solid var(--black);

    &:hover {
      cursor: pointer;
    }
  }
</style>
