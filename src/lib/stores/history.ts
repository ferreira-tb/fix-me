import type { Answer } from './prompt';
import { store, type TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte';

export type History = {
  answers: Answer[];
};

const state: History = {
  answers: [],
};

const options: TauriPluginSvelteStoreOptions = {
  saveOnChange: true,
  syncInterval: 200,
  syncStrategy: 'debounce',
};

export const history = store('history', state, options);
