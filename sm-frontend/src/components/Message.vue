<script setup lang="ts">
const props = defineProps({
  message: String,
  isOurMessage: Boolean
})
</script>

<template>
  <div v-bind:class="(props.isOurMessage)?'flex justify-end':'flex justify-start'">{{props.message}}</div>
</template>

<style>

</style>
