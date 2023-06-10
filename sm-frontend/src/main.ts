import { createApp, watch } from 'vue';
import { createPinia } from 'pinia';
import Toast, { PluginOptions } from "vue-toastification";
import "vue-toastification/dist/index.css";

import App from './App.vue';
import router from './router';

import './assets/main.css';

const app = createApp(App);

const toastOptions: PluginOptions = {
    transition: "Vue-Toastification__bounce",
    maxToasts: 5,
    newestOnTop: true
};

const pinia = createPinia();

// This watcher makes sure that pinia state is persistent across reloads 
watch(
    pinia.state,
    (state) => {
        localStorage.setItem("userData", JSON.stringify(state.userData))
    },
    {deep: true}
)

app.use(pinia);
app.use(router);
app.use(Toast, toastOptions);

app.mount('#app');
