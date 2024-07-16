<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import DebitChip from "@/components/DebitChip.vue";
  import CreditChip from "@/components/CreditChip.vue";
  import { ref, onMounted } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";

  const transactions = ref("");
  const api = useApiStore().api;
  const checkedTransactions = ref([]);
  const allSelected = ref(false);

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
        <tr v-for="transaction in transactions" :id="transaction.transaction.id">
          <td>
            <input
              type="checkbox"
              v-model="checkedTransactions"
              :value="transaction.transaction.id" />
          </td>
          <td>{{ transaction.transaction.desc }}</td>
          <td>{{ new Date(transaction.transaction.date * 1000).toLocaleDateString() }}</td>
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
  }
  button {
    background-color: inherit;
    font-size: 0.8rem;
    width: 15%;
    padding: 5px;
    margin-top: 10px;
    border: 1px solid var(--black);

    &:hover {
      cursor: pointer;
    }
  }
</style>
