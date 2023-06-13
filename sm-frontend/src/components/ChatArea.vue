<script setup lang="ts">
import type { Ref } from "vue";
import { ref } from "vue";
import type { TextMessage } from "../stores/message";
import { useUserStore } from "../stores/user";
import Message from './Message.vue';
const userStore = useUserStore();

const props = defineProps({
  friend: String
})

const MAX_MSG_LENGTH: number = 2000; // TODO: This constraint needs to be added server side
const enteredMessage = ref('');
const messages: Ref<Array<TextMessage>> = ref([]);

function sendMessage() {
  if(enteredMessage.value !== '' && enteredMessage.value.length < MAX_MSG_LENGTH) {
    // TODO: Make an actual call here to send a message
    
    // Var and if/else converts a (string | undefined) to a string
    var receiver: string;
    if(props.friend == undefined) {
      // TODO: Error handle here, can this ever be undefined?
      receiver = '';
    }
    else {
      receiver = props.friend;
    }
    messages.value.push({id: 1, sender: userStore.username, receiver, timestamp: 0, contents: enteredMessage.value});
    enteredMessage.value = '';
  }
}
</script>

<template>
  <div v-if="props.friend !== ''" class="w-9/12 flex flex-col">
    <div class="flex-1">
      <div v-for="message in messages" :key="message.id" class="text-stark-0">
        <Message :message="message.contents" :is-our-message="message.sender === userStore.username"/>
      </div>
    </div>
    <aside class="mb-8">
      <input
          type="text"
          id="enteredMessage"
          v-model="enteredMessage"
          v-on:keyup.enter="sendMessage"
          :placeholder="'Message ' + props.friend"
          class="m-2 p-2 rounded w-7/12"
      />
    </aside>
  </div>
</template>

<style scoped>

</style>