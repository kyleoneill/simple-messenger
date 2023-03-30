<script setup lang="ts">
import { useUserStore } from '../stores/user';
const userStore = useUserStore();
</script>

<template>
  <div class="text-2xl text-stark-0">Settings for {{userStore.username}}</div>
</template>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
