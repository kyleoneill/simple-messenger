<script setup lang="ts">
import {Ref, ref} from "vue";
import { TextMessage } from "../stores/message";
import { useUserStore } from "../stores/user";
const userStore = useUserStore();

const props = defineProps({
  friend: String
})

const MAX_MSG_LENGTH: number = 2000; // TODO: This constraint needs to be added server side
const enteredMessage = ref('');
const messages: Ref<Array<TextMessage>> = ref([]);
const username: string = userStore.username;

function sendMessage() {
  if(enteredMessage.value !== '' && enteredMessage.value.length < MAX_MSG_LENGTH) {
    // TODO: Make an actual call here to send a message
    messages.value.push({id: 1, sender: userStore.username, receiver: props.friend, timestamp: 0, contents: enteredMessage.value});
    enteredMessage.value = '';
  }
}
</script>

<template>
  <div v-if="props.friend !== ''" class="w-9/12 flex flex-col">
    <div class="flex-1">
      <div v-for="message in messages" :key="message.id" class="text-stark-0">
        <div v-if="message.sender === username" class="flex justify-end">
          <div>{{message.contents}}</div>
        </div>
        <div v-else class="flex justify-start">
          <div>{{message.contents}}</div>
        </div>
      </div>
    </div>
    <aside class="mb-8">
      <input
          type="text"
          id="enteredMessage"
          v-model="enteredMessage"
          v-on:keyup.enter="sendMessage"
          :placeholder="'Message ' + props.friend"
          class="m-2 p-2 rounded"
      />
    </aside>
  </div>
</template>

<style scoped>

</style>