import type { Answer } from './prompt.svelte';
import { RuneStore, type TauriPluginSvelteStoreOptions } from '@tauri-store/svelte';

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
