<script setup lang="ts">
import { useUserStore } from '../stores/user';
import {ref} from "vue";
import ChatArea from "../components/ChatArea.vue";
const userStore = useUserStore();
const selectedFriend = ref('');

function selectFriend(selected: string) {
  selectedFriend.value = selected;
}
</script>

<template>
  <main class="flex flex-1 gap-5">
    <div class="border p-2.5 border-fuschia-0/50 border-t-fuschia-0/0 border-b-fuschia-0/0 border-l-fuschia-0/0">
      <div class="text-stark-0 text-2xl">Friends List</div>
      <div class="sticky bottom-0 bg-slate-400/20 h-0.5 mt-2 mb-2" />
      <ul>
        <li @click="selectFriend(friend.username)" v-for="friend in userStore.friends" :key="friend.username" class="text-stark-0">{{friend.username}}</li>
      </ul>
    </div>
    <ChatArea :friend="selectedFriend" />
  </main>
</template>
