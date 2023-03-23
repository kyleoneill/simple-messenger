<script setup lang="ts">
import axios from "axios";
import { ref } from 'vue';
import { RouterLink, RouterView } from 'vue-router'
import { getUserFromStorage, tryCreateUser, tryLogIn, tryLogout, tryGetLocalStorage } from "./stores/user";
import LoginView from "./views/LoginView.vue";

axios.defaults.baseURL = "http://localhost:8080";
axios.defaults.validateStatus = function() {
  return true;
}

const user = ref(getUserFromStorage());

function setUserInfo(newUserName: string, newToken: string) {
  user.value.username = newUserName;
  user.value.token = newToken;
  axios.defaults.headers.common["Authorization"] = newToken;
}

tryGetLocalStorage(setUserInfo);
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
      <LoginView @user-log-in="tryLogIn" @create-new-user="tryCreateUser" :setUserInfo="setUserInfo" />
    </div>
  </div>
</template>

<style src="./assets/main.css"></style>
