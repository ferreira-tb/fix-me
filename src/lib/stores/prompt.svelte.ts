import { RuneStore } from 'tauri-plugin-svelte/runes';
import type { TauriPluginSvelteStoreOptions } from 'tauri-plugin-svelte';

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

const options: TauriPluginSvelteStoreOptions<Prompt> = {
  saveOnChange: true,
  syncInterval: 200,
  syncStrategy: 'debounce',
};

export const prompt = new RuneStore('prompt', state, options);
