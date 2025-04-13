import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useHistoryStore = defineStore('history', () => {
  const answers = ref<Answer[]>([]);

  return {
    answers,
  };
});
