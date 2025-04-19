import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  type Criteria,
  DEFAULT_FORMALITY,
  DEFAULT_GRAMMAR,
  DEFAULT_POLITENESS,
  DEFAULT_READABILITY,
  DEFAULT_TONE,
} from '@/lib/api/bindings';

export const useSettingsStore = defineStore('settings', () => {
  const formality = ref<Criteria>({ message: DEFAULT_FORMALITY, enabled: false });
  const grammar = ref<Criteria>({ message: DEFAULT_GRAMMAR, enabled: false });
  const politeness = ref<Criteria>({ message: DEFAULT_POLITENESS, enabled: false });
  const readability = ref<Criteria>({ message: DEFAULT_READABILITY, enabled: false });
  const tone = ref<Criteria>({ message: DEFAULT_TONE, enabled: false });

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
