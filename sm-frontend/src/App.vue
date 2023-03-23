<script setup lang="ts">
import axios from "axios";
import { ref } from 'vue';
import { RouterLink, RouterView } from 'vue-router'
import { getUser } from "./stores/user";
import { displayToast } from './stores/errorToast';
import LoginView from "./views/LoginView.vue";
import { login, createUser, logout } from './api/user';

axios.defaults.baseURL = "http://localhost:8080";
axios.defaults.validateStatus = function() {
  return true;
}

const user = ref(getUser());

// TODO: Move these functions into the imported "stores/user.ts" file, this is kind of messy here
function setUserInfo(newUserName: string, newToken: string) {
  user.value.username = newUserName;
  user.value.token = newToken;
  axios.defaults.headers.common["Authorization"] = newToken;
}

function tryCreateUser(enteredUsername: string, enteredPassword: string, confirmPassword: string) {
  if(enteredPassword !== '' && enteredPassword !== confirmPassword) {
    displayToast("error", "Entered passwords do not match.");
  }
  else if(enteredUsername !== '' && enteredPassword !== '' && enteredPassword === confirmPassword) {
    createUser(enteredUsername, enteredPassword).then(response => {
      if(response.status === 201) {
        tryLogIn(enteredUsername, enteredPassword);
      }
      else if(response.status === 409) {
        displayToast("error", "That username is already in use.");
      }
      else {
        displayToast("error", "Unknown error when creating a new user.");
      }
    });
  }
}

function tryLogIn(enteredUsername: string, enteredPassword: string) {
  if(enteredUsername !== '' && enteredPassword !== '') {
    login(enteredUsername, enteredPassword).then(response => {
      if(response.status === 201) {
        setLocalStorage(enteredUsername, response.data.token);
        setUserInfo(enteredUsername, response.data.token);
      }
      else {
        displayToast("error", "Invalid username/password.");
      }
    });
  }
}

function setLocalStorage(username: string, token: string) {
  localStorage.setItem("username", username);
  localStorage.setItem("token", token);
}

function tryLogout() {
  logout().then(response => {
    if(response.status !== 200) {
      // TODO: Display toast about failed logout
    }
    localStorage.removeItem("username");
    localStorage.removeItem("token");
    axios.defaults.headers.common["Authorization"] = "";
    location.reload();
  });
}

function tryGetLocalStorage() {
  let username = localStorage.getItem("username");
  let token = localStorage.getItem("token");
  if(username !== null && token !== null) {
    setUserInfo(username, token);
  }
}
tryGetLocalStorage();
</script>

<template>
  <div class="bg-slate-700 min-h-screen m-0 overflow-auto">
    <div v-if="user.token !== ''">
      <header>
        <div class="wrapper">
          <nav class="flex flex-row">
            <RouterLink class="m-3 bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 text-white font-bold py-2 px-4 rounded" to="/">Home</RouterLink>
            <RouterLink class="m-3 bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 text-white font-bold py-2 px-4 rounded" to="/about">About</RouterLink>
            <button @click="tryLogout" class="m-3 bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 text-white font-bold py-2 px-4 rounded">Logout</button>
          </nav>
        </div>
      </header>

      <RouterView />
    </div>
    <div v-else>
      <LoginView @user-log-in="tryLogIn" @create-new-user="tryCreateUser" />
    </div>
  </div>
</template>

<style src="./assets/main.css"></style>
