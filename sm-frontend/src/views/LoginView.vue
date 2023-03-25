<script setup>
import { ref } from 'vue';
defineEmits(['create-new-user', 'user-log-in'])
const enteredUsername = ref('');
const enteredPassword = ref('');
const confirmedPassword = ref('');
const createUser = ref(false);
defineProps(["setUserInfo"]);
// TODO: Add button so that password inbox box can have visible password
// will need some button that toggles the input box type between
// 'text' and 'password'

function swapLoginCreate() {
  createUser.value = !createUser.value;
  enteredUsername.value = '';
  enteredPassword.value = '';
  confirmedPassword.value = '';
}
</script>

<template>
  <div class="ml-10 mt-10">
    <!-- TODO: Make this DRY, the two sections are near duplicates and can be further component-ized -->
    <div v-if="createUser === false">
      <h1 class="text-3xl text-stark-0">Login to Simple Messenger</h1>
      <input
          type="text"
          id="username"
          v-model="enteredUsername"
          placeholder="Username"
          class="m-2 p-2 rounded"
          required
      />
      <br />
      <input
          type="password"
          id="password"
          v-model="enteredPassword"
          placeholder="Password"
          class="m-2 p-2 rounded"
          required
      />
      <br />
      <button @click="$emit('user-log-in', enteredUsername, enteredPassword, setUserInfo)" class="m-3 bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 text-white font-bold py-2 px-4 rounded">Login</button>
      <br />
      <a class="text-stark-0 no-underline hover:underline cursor-pointer" @click="swapLoginCreate">Create a user</a>
    </div>
    <div v-else>
      <h1 class="text-3xl text-stark-0">Create a user</h1>
      <input
          type="text"
          id="username"
          v-model="enteredUsername"
          placeholder="Username"
          class="m-2 p-2 rounded"
          required
      />
      <br />
      <input
          type="password"
          id="password"
          v-model="enteredPassword"
          placeholder="Password"
          class="m-2 p-2 rounded"
          required
      />
      <br />
      <input
          type="password"
          id="confirmPassword"
          v-model="confirmedPassword"
          placeholder="Confirm password"
          class="m-2 p-2 rounded"
          required
      />
      <br />
      <button @click="$emit('create-new-user', enteredUsername, enteredPassword, confirmedPassword, setUserInfo)" class="m-3 bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 text-white font-bold py-2 px-4 rounded">Create User</button>
      <br />
      <a class="text-stark-0 hover:underline cursor-pointer" @click="swapLoginCreate">Go back to login</a>
    </div>
  </div>
</template>

<style src="../assets/main.css"></style>
