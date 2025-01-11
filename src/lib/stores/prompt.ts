import { store, type TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte';

export type Prompt = {
  answer: Answer | null;
  message: string;
};

export type Answer = {
  date: number;
  text: string;
};

const state: Prompt = {
  answer: null,
  message: '',
};

const options: TauriPluginSvelteStoreOptions = {
  saveOnChange: true,
  syncInterval: 200,
  syncStrategy: 'debounce',
};

export const prompt = store('prompt', state, options);
