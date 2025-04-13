import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useSettingsStore = defineStore('settings', () => {
  const formality = ref(false);
  const grammar = ref(false);
  const politeness = ref(false);
  const readability = ref(false);
  const tone = ref(false);

  const token = ref('');

  return {
    formality,
    grammar,
    politeness,
    readability,
    tone,
    token,
  };
});
