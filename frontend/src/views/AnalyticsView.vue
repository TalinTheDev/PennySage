<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";
  import { ref, onMounted } from "vue";
  import Card from "@/components/Card.vue";
  import Chart from "primevue/chart";

  import { useApiStore } from "@/stores/useApiStore.js";
  import { useToast } from "primevue/usetoast";

  const api = useApiStore().api;
  const toast = useToast();

  let monthly_debit_balance_for_last_6_months = ref({ labels: [], datasets: [] });
  let net_balance_for_past_5_years = ref({ labels: [], datasets: [] });

  let barChartOptions = ref({
    responsive: true,
    maintainAspectRatio: false,
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
      .get("/analytics/analytics")
      .then((response) => {
        /*
         * Reversing done for display purposes (latest data on the right)
         * Can't be done by using a different ordering in SQL query as that results in opposite end of dates being
         * fetched
         */
        monthly_debit_balance_for_last_6_months.value.datasets.push({
          label: " Debit Balance", // Space needed in front of label for formatting
          data: []
        });
        response.monthly_debit_balance_for_last_6_months.reverse().forEach((item) => {
          monthly_debit_balance_for_last_6_months.value.labels.push(
            new Date(item.date).toLocaleString("default", { month: "long" })
          );
          monthly_debit_balance_for_last_6_months.value.datasets[0].data.push(item.debit_balance);
        });

        net_balance_for_past_5_years.value.datasets.push({
          label: " Balance", // Space needed in front of label for formatting
          data: []
        });
        response.net_balance_for_past_5_years.reverse().forEach((item) => {
          net_balance_for_past_5_years.value.labels.push(item.year);
          net_balance_for_past_5_years.value.datasets[0].data.push(item.balance);
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
        <span>Analytics</span>
      </template>
    </PageHeader>
    <div class="charts">
      <Card class="card mdbflsm" isCentered>
        <template #header>
          <span>Monthly debit balance for the last six months</span>
        </template>
        <template #content>
          <Chart
            v-if="monthly_debit_balance_for_last_6_months.labels.length != 0"
            type="bar"
            :data="monthly_debit_balance_for_last_6_months"
            :options="barChartOptions"
            :pt="{ root: { class: 'chartContainer' } }" />
          <p v-else>No data available</p>
        </template>
      </Card>
      <Card class="card nbfpfy" isCentered>
        <template #header>
          <span>Net balance at the end of the year for the past five years</span>
        </template>
        <template #content>
          <Chart
            v-if="net_balance_for_past_5_years.labels.length != 0"
            type="bar"
            :data="net_balance_for_past_5_years"
            :options="barChartOptions"
            :pt="{ root: { class: 'chartContainer' } }" />
          <p v-else>No data available</p>
        </template>
      </Card>
    </div>
  </AppLayout>
</template>

<style scoped>
  .charts {
    display: flex;
  }

  .chartContainer {
    display: flex;
    flex-flow: column;
    width: 100%;
    height: 100%;
  }
</style>
