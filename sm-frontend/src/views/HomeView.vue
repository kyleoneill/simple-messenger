<script setup lang="ts">
import { getRelationships } from "../api/relationships";
let friendsList = [];
getRelationships().then(response => {
  if(response.status === 200) {
    for (const i in response.data) {
      friendsList.push(response.data[i]);
    }
  }
  console.log(response);
});
</script>

<template>
  <main>
    <ul>
      <li v-for="friend in friendsList" :key="friend.username">
        {{friend.username}}
      </li>
    </ul>
  </main>
</template>
