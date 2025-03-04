import type { Answer } from './prompt.svelte';
import { RuneStore } from 'tauri-plugin-svelte/runes';
import type { TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte';

export type History = {
  answers: Answer[];
};

const state: History = {
  answers: [],
};

const options: TauriPluginSvelteStoreOptions<History> = {
  saveOnChange: true,
  syncInterval: 200,
  syncStrategy: 'debounce',
};

export const history = new RuneStore('history', state, options);
