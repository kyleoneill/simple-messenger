<script setup lang="ts">
// TODO: clean up imports, I should be using some kind of auto-formatter
import axios from "axios";
import { ref } from 'vue';
import { RouterLink, RouterView } from 'vue-router'
import { getUserFromStorage, tryCreateUser, tryLogIn, tryLogout, tryGetLocalStorage, useUserStore } from "./stores/user";
import LoginView from "./views/LoginView.vue";
import { getFriends } from './api/relationships';

axios.defaults.baseURL = "http://localhost:8080";
axios.defaults.validateStatus = function() { return true; }

const user = ref(getUserFromStorage());

async function initializeUserState(newUserName: string, newToken: string) {
  axios.defaults.headers.common["Authorization"] = newToken;
  const userState = useUserStore();
  let friends = await getFriends();
  userState.updateUsername(newUserName);
  userState.setFriends(friends);

  user.value.username = newUserName;
  user.value.token = newToken;
}

tryGetLocalStorage(initializeUserState);
</script>

<template>
  <div class="bg-void-0 min-h-screen m-0 overflow-auto">
    <div class="mx-10 h-screen">
      <div class="h-screen flex flex-col" v-if="user.token !== ''">
        <header class="sticky w-full top-0">
          <div class="wrapper">
            <nav class="flex flex-row items-center">
              <RouterLink class="m-3 text-stark-0 cursor-pointer" to="/">Home</RouterLink>
              <RouterLink class="m-3 text-stark-0 cursor-pointer" to="/settings">Settings</RouterLink>
              <button @click="tryLogout" class="m-3 text-stark-0 cursor-pointer">Logout</button>
            </nav>
          </div>
        </header>
        <br class="visible" />

        <div class="flex flex-1">
          <RouterView />
        </div>
      </div>
      <div v-else>
        <LoginView @user-log-in="tryLogIn" @create-new-user="tryCreateUser" :setUserInfo="initializeUserState" />
      </div>
    </div>
  </div>
</template>

<style src="./assets/main.css"></style>
