import '@/assets/index.css';
import '@tb-dev/vue-components/style';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { router } from '@/router';
import { createPinia } from 'pinia';
import { onError } from '@/lib/utils';
import { TauriPluginPinia } from '@tauri-store/pinia';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);
const pinia = createPinia();

pinia.use(
  TauriPluginPinia({
    saveOnChange: true,
  })
);

setCurrentApp(app);
setErrorHandler(onError, app);

app.use(router);
app.use(pinia);

// prettier-ignore
void router
  .push({ name: 'home' satisfies Route })
  .then(() => app.mount('#app'));
