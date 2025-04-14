<script setup>
  import AppLayout from "@/layouts/AppLayout.vue";
  import PageHeader from "@/components/PageHeader.vue";
  import InputText from "primevue/inputtext";
  import Password from "primevue/password";
  import InputBox from "@/components/InputBox.vue";
  import Card from "@/components/Card.vue";
  import Button from "@/components/Button.vue";

  import { ref, provide } from "vue";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useUserStore } from "@/stores/useUserStore";
  import { useToast } from "primevue/usetoast";
  import { useConfirm } from "primevue/useconfirm";
  import router from "@/router";

  const toast = useToast();
  const confirm = useConfirm();
  const api = useApiStore().api;
  const userStore = useUserStore();
  const errors = ref(new Map());
  const oldPassword = ref("");
  const newPassword = ref("");

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

  const compareUserDetails = (oldD, newD) => {
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
    if (
      errors.value.size !== 0 &&
      !(errors.value.get("Old PasswordErr") || errors.value.get("New PasswordErr"))
    ) {
      return;
    }
    let edits = compareUserDetails(oldDetails.value, newDetails.value);
    if (edits.length > 0) {
      api
        .patch("/settings/editUser", {
          edits: edits
        })
        .then((data) => {
          toast.add({
            severity: "info",
            summary: "User Details Edited",
            detail: "Details successfully edited",
            life: 3000
          });

          userStore.login(data.user_details, data.access_token);
          oldDetails.value = {
            username: userStore.username,
            email: userStore.email,
            firstName: userStore.firstName,
            lastName: userStore.lastName
          };
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
    } else {
      toast.add({
        severity: "info",
        summary: "User Details Editing",
        detail: "Nothing to update",
        life: 3000
      });
    }
  };

  const updatePassword = () => {
    if (
      (errors.value.size !== 0 &&
        !(
          errors.value.get("UsernameErr") ||
          errors.value.get("EmailErr") ||
          errors.value.get("First NameErr") ||
          errors.value.get("Last NameErr")
        )) ||
      newPassword.value == "" ||
      oldPassword.value == ""
    ) {
      return;
    }
    api
      .patch("/settings/updatePassword", {
        old: oldPassword.value,
        new: newPassword.value
      })
      .then(() => {
        toast.add({
          severity: "info",
          summary: "Password Updated",
          detail: "Password successfully updated",
          life: 3000
        });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Password Update Request Failed",
          detail: error,
          life: 3000
        });
        newDetails.value = oldDetails.value;
      });
  };

  const deleteAccount = () => {
    confirm.require({
      message: "Are you sure you want to delete your account?",
      header: "Delete Account",
      icon: "pi pi-exclamation-triangle",
      rejectProps: {
        label: "No",
        severity: "secondary",
        outlined: true
      },
      acceptProps: {
        label: "Delete",
        severity: "danger"
      },
      accept: () => {
        api
          .delete("/auth/user")
          .then(() => {
            router.push("/");
            userStore.logout();
            toast.add({
              severity: "info",
              summary: "Account Deleted",
              detail: "Account successfully deleted",
              life: 3000
            });
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
    <Card>
      <template #header>
        <span class="formHeader">Security</span>
      </template>
      <template #content>
        <InputBox name="Old Password">
          <Password
            fluid
            v-model="oldPassword"
            :feedback="false"
            required
            placeholder="Old Password"
            @blur="
              (event) => {
                validate(event.target, 'Old Password');
              }
            " />
        </InputBox>
        <InputBox name="New Password">
          <Password
            fluid
            v-model="newPassword"
            :feedback="false"
            required
            placeholder="New Password"
            @blur="
              (event) => {
                validate(event.target, 'New Password');
              }
            " />
        </InputBox>
        <Button @click="updatePassword">Update Password</Button>
      </template>
    </Card>
    <Card>
      <template #header>
        <span class="formHeader">Account Actions</span>
      </template>
      <template #content>
        <Button @click="deleteAccount">Delete Account</Button>
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
