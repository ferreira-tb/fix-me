import { store, type TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte';

export type Settings = {
  formality: boolean;
  grammar: boolean;
  politeness: boolean;
  readability: boolean;
  token: string | null;
  tone: boolean;
};

const state: Settings = {
  token: null,
  grammar: false,
  readability: false,
  tone: false,
  politeness: false,
  formality: false,
};

const options: TauriPluginSvelteStoreOptions<Settings> = {
  saveOnChange: true,
};

export const settings = store('settings', state, options);
