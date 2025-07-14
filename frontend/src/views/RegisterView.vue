// TODO: Add family option

<script setup>
  import { ref, provide } from "vue";
  import { RouterLink } from "vue-router";
  import InputBox from "@/components/InputBox.vue";
  import Card from "@/components/Card.vue";
  import Button from "@/components/Button.vue";
  import AuthLayout from "../layouts/AuthLayout.vue";
  import InputText from "primevue/inputtext";
  import Password from "primevue/password";
  import { useApiStore } from "@/stores/useApiStore.js";
  import { useUserStore } from "@/stores/useUserStore.js";
  import { useRouter } from "vue-router";
  import { useToast } from "primevue/usetoast";

  const toast = useToast();
  const api = useApiStore().api;
  const userStore = useUserStore();
  const router = useRouter();

  const username = ref("");
  const email = ref("");
  const firstName = ref("");
  const lastName = ref("");
  const password = ref("");
  const confirmPassword = ref("");

  const errors = ref(new Map());

  const validate = (target, name) => {
    if (!target.checkValidity()) {
      errors.value.set(`${name}Err`, target.validationMessage);
    } else {
      if (name == "Confirm Password") {
        if (password.value !== confirmPassword.value) {
          errors.value.set(`${name}Err`, "Passwords do not match");
        } else {
          errors.value.delete(`${name}Err`);
        }
      } else {
        errors.value.delete(`${name}Err`);
      }
    }
  };
  provide("errors", errors);

  const checkEmptyValues = () => {
    if (username.value == "") {
      errors.value.set("UsernameErr", "Please enter your username");
    } else {
      errors.value.delete("UsernameErr");
    }
    if (email.value == "") {
      errors.value.set("EmailErr", "Please enter your email address");
    } else {
      if (!errors.value.has("EmailErr")) {
        errors.value.delete("EmailErr");
      }
    }
    if (firstName.value == "") {
      errors.value.set("First NameErr", "Please enter your first name");
    } else {
      errors.value.delete("First NameErr");
    }
    if (lastName.value == "") {
      errors.value.set("Last NameErr", "Please enter your last name");
    } else {
      errors.value.delete("Last NameErr");
    }
    if (password.value == "") {
      errors.value.set("PasswordErr", "Please enter your password");
    } else {
      errors.value.delete("PasswordErr");
    }
    if (confirmPassword.value == "") {
      errors.value.set("Confirm PasswordErr", "Please confirm your password");
    } else {
      if (!errors.value.has("Confirm PasswordErr")) {
        errors.value.delete("Confirm PasswordErr");
      }
    }
  };

  const handleRegister = () => {
    if (password.value !== confirmPassword.value) {
      errors.value.set("Confirm PasswordErr", "Passwords do not match");
    }
    checkEmptyValues();
    if (errors.value.size !== 0) {
      return;
    }
    api
      .post("/auth/register", {
        username: username.value,
        email: email.value,
        first_name: firstName.value,
        last_name: lastName.value,
        password: password.value
      })
      .then(({ data }) => {
        userStore.login(data.user_details, data.access_token);
        router.push({ name: "dashboard" });
      })
      .catch((error) => {
        toast.add({
          severity: "error",
          summary: "Error registering",
          detail: error.message,
          life: 3000
        });
      });
  };
</script>

<template>
  <AuthLayout>
    <Card>
      <template #header>
        <h1>Register <RouterLink to="login" class="loginLink">Or Login</RouterLink></h1>
      </template>
      <template #content>
        <InputBox name="Username">
          <InputText
            fluid
            v-model="username"
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
            v-model="email"
            placeholder="Email"
            required
            pt:root:type="email"
            @blur="
              (event) => {
                validate(event.target, 'Email');
              }
            " />
        </InputBox>
        <div class="columns">
          <InputBox name="First Name">
            <InputText
              fluid
              v-model="firstName"
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
              v-model="lastName"
              placeholder="Last Name"
              required
              @blur="
                (event) => {
                  validate(event.target, 'Last Name');
                }
              " />
          </InputBox>
        </div>
        <div class="columns">
          <InputBox name="Password">
            <Password
              fluid
              v-model="password"
              :feedback="false"
              required
              placeholder="Password"
              @blur="
                (event) => {
                  validate(event.target, 'Password');
                }
              " />
          </InputBox>
          <InputBox name="Confirm Password">
            <Password
              fluid
              v-model="confirmPassword"
              :feedback="false"
              required
              placeholder="Confirm Password"
              @keydown.enter="handleRegister"
              @blur="
                (event) => {
                  validate(event.target, 'Confirm Password');
                }
              " />
          </InputBox>
        </div>
        <Button @click="handleRegister">Register</Button>
      </template>
    </Card>
  </AuthLayout>
</template>

<style scoped>
  .columns {
    display: flex;
    column-gap: 1rem;
  }

  .loginLink {
    font-size: 0.75rem;
    color: inherit;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
      text-underline-offset: 50%;
    }
  }
</style>
