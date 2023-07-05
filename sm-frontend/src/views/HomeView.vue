<script setup lang="ts">
import { useUserStore } from '../stores/user';
import {ref} from "vue";
import ChatArea from "../components/ChatArea.vue";
const userStore = useUserStore();
const selectedFriend = ref('');

function selectFriend(selected: string) {
  selectedFriend.value = selected;
}

// If the user has any friends, automatically open a chat with the first one in our friends list
let friends = userStore.friendsList;
if(friends.length > 0) {
  selectedFriend.value = friends[0].username;
}
</script>

<template>
  <main class="flex flex-1 gap-5">
    <div class="border p-2.5 border-fuschia-0/50 border-t-fuschia-0/0 border-b-fuschia-0/0 border-l-fuschia-0/0">
      <div class="text-stark-0 text-2xl">Friends List</div>
      <div class="sticky bottom-0 bg-slate-400/20 h-0.5 mt-2 mb-2" />
      <ul>
        <li @click="selectFriend(friend.username)" v-for="friend in userStore.friends" :key="friend.username" class="text-stark-0 cursor-pointer">{{friend.username}}</li>
      </ul>
    </div>
    <ChatArea :friend="selectedFriend" />
  </main>
</template>

<style>
  .sm-btn-3d {
    @apply bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-pink-300 rounded
  }
  .sm-btn {
    @apply m-3 bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 text-white font-bold py-2 px-4 rounded
  }
  .sm-white-link-text {
    @apply text-stark-0 hover:underline cursor-pointer
  }
</style>
