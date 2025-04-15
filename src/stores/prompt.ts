import { ref } from 'vue';
import { defineStore } from 'pinia';

export const usePromptStore = defineStore('prompt', () => {
  const message = ref('');
  const answer = ref<Answer | null>(null);

  return {
    message,
    answer,
  };
});
