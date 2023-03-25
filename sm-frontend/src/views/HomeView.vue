<script setup lang="ts">
import { useUserStore } from '../stores/user';
const userStore = useUserStore();
</script>

<template>
  <main>
    <div>{{userStore.username}}</div>
    <ul>
      <li v-for="friend in userStore.friends" :key="friend">
        {{friend}}
      </li>
    </ul>
  </main>
</template>
