<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import { ref } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";

  const transactions = ref("");
  const api = useApiStore().api;
  const checkedTransactions = ref([]);
  const allSelected = ref(false);

  api
    .get("/transactions/all")
    .then((data) => {
      transactions.value = data;
      console.log(data);
    })
    .catch((error) => {
      alert(`Error: ${JSON.stringify(error.message)}`);
    });

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
</script>
<template>
  <AppLayout>
    <h2>View Transactions</h2>
    <table class="table">
      <thead>
        <tr>
          <th><input type="checkbox" @click="checkAll" v-model="allSelected" /></th>
          <th>Description</th>
          <th>Date</th>
          <th>Amount</th>
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
          <td>${{ transaction.transaction.amount }}</td>
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
</style>
