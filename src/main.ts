import '@/assets/style/base.css';
import '@/assets/style/main.css';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { router } from '@/router';
import { createPinia } from 'pinia';
import { handleError } from '@/lib/error';
import { checkForUpdates } from '@/lib/updater';
import { TauriPluginPinia } from '@tauri-store/pinia';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);
const pinia = createPinia();

pinia.use(
  TauriPluginPinia({
    autoStart: true,
    saveOnChange: true,
  }),
);

setCurrentApp(app);
setErrorHandler(handleError, app);

app.use(router);
app.use(pinia);

try {
  await checkForUpdates();
}
catch (err) {
  handleError(err);
}

app.mount('#app');
