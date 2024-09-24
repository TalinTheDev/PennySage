<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";
  import InputText from "primevue/inputtext";
  import InputBox from "@/components/InputBox.vue";
  import Card from "@/components/Card.vue";
  import Button from "@/components/Button.vue";

  import { ref, provide } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useUserStore } from "@/stores/useUserStore";
  import { useToast } from "primevue/usetoast";

  const toast = useToast();
  const api = useApiStore().api;
  const userStore = useUserStore();
  const errors = ref(new Map());

  provide("errors", errors);

  let oldDetails = ref({
    username: userStore.username,
    email: userStore.email,
    firstName: userStore.firstName,
    lastName: userStore.lastName
  });
  let newDetails = ref({
    username: userStore.username,
    email: userStore.email,
    firstName: userStore.firstName,
    lastName: userStore.lastName
  });

  const validate = (target, name) => {
    if (!target.checkValidity()) {
      errors.value.set(`${name}Err`, target.validationMessage);
    } else {
      errors.value.delete(`${name}Err`);
    }
  };

  const compareTransactions = (oldD, newD) => {
    let diff = [];

    if (oldD.username !== newD.username) {
      diff.push({ field: "username", value: newD.username });
    }
    if (oldD.email !== newD.email) {
      diff.push({ field: "email", value: newD.email });
    }
    if (oldD.firstName !== newD.firstName) {
      diff.push({ field: "firstName", value: newD.firstName });
    }
    if (oldD.lastName !== newD.lastName) {
      diff.push({ field: "lastName", value: newD.lastName });
    }

    return diff;
  };

  const editUser = () => {
    let edits = compareTransactions(oldDetails.value, newDetails.value);
    api
      .patch("/settings/editUser", {
        edits: edits
      })
      .then((data) => {
        toast.add({
          severity: "info",
          summary: "User Details Edited",
          detail: "Details edited successfully",
          life: 3000
        });

        userStore.login(data.user_details, data.access_token);
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "User Detail Edit Failed",
          detail: error.message,
          life: 3000
        });
        newDetails.value = oldDetails.value;
      });
  };
</script>
<template>
  <AppLayout>
    <PageHeader>
      <template #header>
        <span>Settings</span>
      </template>
    </PageHeader>
    <Card>
      <template #header>
        <span class="formHeader">User Details</span>
      </template>
      <template #content>
        <div class="columns">
          <InputBox name="Username">
            <InputText
              fluid
              id="username"
              v-model="newDetails.username"
              placeholder="Username"
              required
              @blur="
                (event) => {
                  validate(event.target, 'Username');
                }
              " />
          </InputBox>

          <InputBox name="Email">
            <InputText
              fluid
              v-model="newDetails.email"
              placeholder="Email"
              required
              pt:root:type="email"
              @blur="
                (event) => {
                  validate(event.target, 'Email');
                }
              " />
          </InputBox>
        </div>
        <div class="columns">
          <InputBox name="First Name">
            <InputText
              fluid
              id="firstName"
              v-model="newDetails.firstName"
              placeholder="First Name"
              required
              @blur="
                (event) => {
                  validate(event.target, 'First Name');
                }
              " />
          </InputBox>

          <InputBox name="Last Name">
            <InputText
              fluid
              id="lastName"
              v-model="newDetails.lastName"
              placeholder="Last Name"
              required
              @blur="
                (event) => {
                  validate(event.target, 'Last Name');
                }
              " />
          </InputBox>
        </div>
        <Button @click="editUser">Save User Details</Button>
      </template>
    </Card>
  </AppLayout>
</template>
<style scoped>
  .columns {
    display: flex;
    column-gap: 1rem;
  }
</style>
