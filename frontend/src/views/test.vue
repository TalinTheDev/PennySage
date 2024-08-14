<template>
  <div class="card">
    <DataTable
      v-model:editingRows="editingRows"
      :value="products"
      editMode="row"
      dataKey="id"
      @row-edit-save="onRowEditSave">
      <Column field="desc" header="Desc" style="width: 20%">
        <template #editor="{ data, field }">
          <InputText v-model="data[field]" fluid />
        </template>
      </Column>
      <Column field="merchant" header="Merchant" style="width: 20%">
        <template #editor="{ data, field }">
          <InputText v-model="data[field]" fluid />
        </template>
      </Column>
      <Column
        :rowEditor="true"
        style="width: 10%; min-width: 8rem"
        bodyStyle="text-align:center"></Column>
    </DataTable>
  </div>
</template>

<script setup>
  import DataTable from "primevue/datatable";
  import Column from "primevue/column";
  import InputText from "primevue/inputtext";
  import { ref, onMounted } from "vue";

  const products = ref();
  const editingRows = ref([]);
  onMounted(() => {
    products.value = [
      {
        amount: "$1.00",
        category: "A",
        date: "2024-07-17T04:00:00.000Z",
        desc: "Edited description goes brrrrrrrrrrrrrrrrrr",
        id: "KXCYHwe76xDmdfH6Q4PR1",
        items: [],
        merchant: "A",
        payment_source: "A",
        type: "debit"
      },
      {
        amount: "$2.00",
        category: "B",
        date: "2024-07-17T04:00:00.000Z",
        desc: "B",
        id: "KJWvfU6ceaMuMQ_CBxjDJ",
        items: [],
        merchant: "B",
        payment_source: "B",
        type: "debit"
      }
    ];
  });

  const onRowEditSave = (event) => {
    let { newData, index } = event;
    products.value[index] = newData;
    console.log(JSON.stringify(newData, null, 2));
  };
</script>
