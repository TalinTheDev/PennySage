<script setup>
  import { ref, onMounted } from "vue";
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";
  import Card from "@/components/Card.vue";
  import Chart from "primevue/chart";

  import { useUserStore } from "@/stores/useUserStore.js";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useToast } from "primevue/usetoast";

  const api = useApiStore().api;
  const user = useUserStore();
  const toast = useToast();

  let date = new Date();
  date = ref(
    new Intl.DateTimeFormat("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric"
    }).format(date)
  );

  let amount_per_category_current_month = ref({ labels: [], datasets: [] });
  let top_categories_for_amount_spent_for_current_month = ref({ labels: [], datasets: [] });
  let current_month_balance_compared_to_last_year = ref({ labels: [], datasets: [] });
  let last_months_balance_by_source = ref([]);
  let current_months_balance_by_source = ref([]);
  let current_months_credit_debit_balance = ref([]);
  let average_savings = ref({ labels: [], datasets: [] });

  let pieChartOptions = ref({
    responsive: true,
    plugins: {
      tooltip: {
        callbacks: {
          label: function (context) {
            let label = context.dataset.label || "";

            if (label) {
              label += ": ";
            }
            if (context.parsed.y !== null) {
              label += new Intl.NumberFormat("en-US", {
                style: "currency",
                currency: "USD"
              }).format(context.parsed);
            }
            return label;
          }
        }
      }
    }
  });
  let barChartOptions = ref({
    responsive: true,
    plugins: {
      tooltip: {
        callbacks: {
          label: function (context) {
            let label = context.dataset.label || "";

            if (label) {
              label += ": ";
            }
            if (context.parsed.y !== null) {
              label += new Intl.NumberFormat("en-US", {
                style: "currency",
                currency: "USD"
              }).format(context.parsed.y);
            }
            return label;
          }
        }
      }
    }
  });

  const getAnalyticsData = () => {
    api
      .get("/analytics/dashboard")
      .then((response) => {
        amount_per_category_current_month.value.datasets.push({
          label: " Balance", // Space needed in front of label for formatting
          data: []
        });
        response.amount_per_category_current_month.forEach((item) => {
          amount_per_category_current_month.value.labels.push(item.category);
          amount_per_category_current_month.value.datasets[0].data.push(item.balance);
        });

        top_categories_for_amount_spent_for_current_month.value.datasets.push({
          label: " Amount Spent", // Space needed in front of label for formatting
          data: []
        });
        response.top_categories_for_amount_spent_for_current_month.forEach((item) => {
          top_categories_for_amount_spent_for_current_month.value.labels.push(item.category);
          top_categories_for_amount_spent_for_current_month.value.datasets[0].data.push(
            item.amount
          );
        });

        current_month_balance_compared_to_last_year.value.datasets.push({
          label: " Balance", // Space needed in front of label for formatting
          data: []
        });
        response.current_month_balance_compared_to_last_year.forEach((item) => {
          current_month_balance_compared_to_last_year.value.labels.push(item.year);
          current_month_balance_compared_to_last_year.value.datasets[0].data.push(item.balance);
        });

        response.last_months_balance_by_source.forEach((item) => {
          last_months_balance_by_source.value.push({
            source: item.source,
            balance: `${parseFloat(item.balance) < 0 ? "-" : ""}$${Math.abs(parseFloat(item.balance)).toFixed(2)}`
          });
        });

        response.current_months_balance_by_source.forEach((item) => {
          current_months_balance_by_source.value.push({
            source: item.source,
            balance: `${parseFloat(item.balance) < 0 ? "-" : ""}$${Math.abs(parseFloat(item.balance)).toFixed(2)}`
          });
        });

        let credit_debit_balance = response.current_months_credit_debit_balance;
        current_months_credit_debit_balance.value.push({
          credit: `$${parseFloat(credit_debit_balance.credit).toFixed(2)}`,
          debit: `$${parseFloat(credit_debit_balance.debit).toFixed(2)}`,
          balance: `$${Math.abs(parseFloat(credit_debit_balance.balance)).toFixed(2)}`
        });

        average_savings.value.datasets.push({
          label: " Savings", // Space needed in front of label for formatting
          data: []
        });
        response.average_savings.reverse().forEach((item) => {
          average_savings.value.labels.push(
            Intl.DateTimeFormat("en", { year: "numeric", month: "short" }).format(
              new Date(item.year, item.month - 1)
            )
          );
          average_savings.value.datasets[0].data.push(item.savings);
        });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Analytics data could not be fetched",
          detail: error.message,
          life: 3000
        });
      });
  };
  onMounted(() => {
    getAnalyticsData();
  });
</script>
<template>
  <AppLayout>
    <PageHeader>
      <template #header>
        <span>Hey, {{ user.firstName }}</span>
      </template>
      <template #subheader>
        <span>{{ date }}</span>
      </template>
    </PageHeader>
    <div class="charts">
      <Card class="card bpctm" isCentered>
        <template #header>
          <span>Balance per category this month</span>
        </template>
        <template #content>
          <Chart
            v-if="amount_per_category_current_month.labels.length != 0"
            type="pie"
            :data="amount_per_category_current_month"
            :options="pieChartOptions"
            :canvasProps="{ style: 'flex: 1 1 0;height: auto;' }"
            :pt="{ root: { class: 'chartContainer' } }" />
          <p v-else>No data available</p>
        </template>
      </Card>
      <Card class="card asitctm" isCentered>
        <template #header>
          <span>Amount spent in top categories this month</span>
        </template>
        <template #content>
          <Chart
            v-if="top_categories_for_amount_spent_for_current_month.labels.length != 0"
            type="bar"
            :data="top_categories_for_amount_spent_for_current_month"
            :options="barChartOptions"
            :canvasProps="{ style: 'flex: 1 1 0;height: auto;' }"
            :pt="{ root: { class: 'chartContainer' } }" />
          <p v-else>No data available</p>
        </template>
      </Card>
      <Card class="card cmbacttmly" isCentered>
        <template #header>
          <span>Current month's balance as compared to this month last year</span>
        </template>
        <template #content>
          <Chart
            v-if="current_month_balance_compared_to_last_year.labels.length != 0"
            type="bar"
            :data="current_month_balance_compared_to_last_year"
            :options="barChartOptions"
            :canvasProps="{ style: 'flex: 1 1 0;height: auto;' }"
            :pt="{ root: { class: 'chartContainer' } }" />
          <p v-else>No data available</p>
        </template>
      </Card>
      <Card class="card lmbcbs" isCentered>
        <template #header>
          <span>Last month's balance categorized by source</span>
        </template>
        <template #content>
          <table v-if="last_months_balance_by_source.length != 0">
            <tr>
              <th>Source</th>
              <th>Balance</th>
            </tr>
            <tr v-for="item in last_months_balance_by_source" :key="item.source">
              <td>{{ item.source }}</td>
              <td>{{ item.balance }}</td>
            </tr>
          </table>
          <p v-else>No data available</p>
        </template>
      </Card>
      <Card class="card cmbcbs" isCentered>
        <template #header>
          <span>Current month's balance categorized by source</span>
        </template>
        <template #content>
          <table v-if="current_months_balance_by_source.length != 0">
            <tr>
              <th>Source</th>
              <th>Balance</th>
            </tr>
            <tr v-for="item in current_months_balance_by_source" :key="item.source">
              <td>{{ item.source }}</td>
              <td>{{ item.balance }}</td>
            </tr>
          </table>
          <p v-else>No data available</p>
        </template>
      </Card>
      <Card class="card cmcvdt" isCentered>
        <template #header>
          <span>Current month's credit vs debit transactions</span>
        </template>
        <template #content>
          <table v-if="current_months_credit_debit_balance.length != 0">
            <tr>
              <th>Credit</th>
              <th>Debit</th>
              <th>Balance</th>
            </tr>
            <tr v-for="item in current_months_credit_debit_balance" :key="item.credit">
              <td>{{ item.credit }}</td>
              <td>{{ item.debit }}</td>
              <td>{{ item.balance }}</td>
            </tr>
          </table>
          <p v-else>No data available</p>
        </template>
      </Card>
      <Card class="card aspmftlsm" isCentered>
        <template #header>
          <span>Average savings per month for the last six months</span>
        </template>
        <template #content>
          <Chart
            v-if="average_savings.labels.length != 0"
            type="bar"
            :data="average_savings"
            :options="barChartOptions"
            :canvasProps="{ style: 'flex: 1 1 0;height: auto;' }"
            :pt="{ root: { class: 'chartContainer' } }" />
          <p v-else>No data available</p>
        </template>
      </Card>
    </div>
  </AppLayout>
</template>

<style scoped>
  .charts {
    display: grid;
    grid-template-columns: repeat(12, 1fr);
    /* grid-template-rows: repeat(3, 1fr); */
    grid-template-areas:
      "bpctm bpctm bpctm bpctm bpctm lmbcbs lmbcbs lmbcbs lmbcbs cmbcbs cmbcbs cmbcbs"
      "asitctm asitctm asitctm asitctm cmbacttmly cmbacttmly cmbacttmly cmbacttmly aspmftlsm aspmftlsm aspmftlsm aspmftlsm"
      ". . . cmcvdt cmcvdt cmcvdt cmcvdt cmcvdt cmcvdt . . .";
  }

  tr,
  th,
  td,
  table {
    border: 1px solid var(--dark-grey);
    border-collapse: collapse;
    padding: 1rem;
    width: 100%;
    text-align: center;
  }

  .bpctm {
    grid-area: bpctm;
  }

  .asitctm {
    grid-area: asitctm;
  }

  .cmbacttmly {
    grid-area: cmbacttmly;
  }

  .aspmftlsm {
    grid-area: aspmftlsm;
  }

  .lmbcbs {
    grid-area: lmbcbs;
  }

  .cmbcbs {
    grid-area: cmbcbs;
  }

  .cmcvdt {
    grid-area: cmcvdt;
  }

  .chartContainer {
    display: flex;
    flex-flow: column;
    width: 100%;
    height: 100%;
  }
</style>
