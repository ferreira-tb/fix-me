<script setup lang="ts">
import { computed, ref } from 'vue';
import { storeToRefs } from 'pinia';
import { onError } from '@/lib/utils';
import { commands } from '@/lib/api/bindings';
import { usePromptStore } from '@/stores/prompt';
import { useHistoryStore } from '@/stores/history';
import { useSettingsStore } from '@/stores/settings';
import { Button, Textarea } from '@tb-dev/vue-components';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const promptStore = usePromptStore();
const { message, answer } = storeToRefs(promptStore);

const historyStore = useHistoryStore();
const { answers } = storeToRefs(historyStore);

const settings = useSettingsStore();

const loading = ref(false);
const disabled = computed(() => {
  return loading.value || !message.value || !settings.token;
});

async function fix() {
  try {
    loading.value = true;
    const answerText = await commands.prompt(message.value);
    answer.value = { text: answerText, date: Date.now() };

    const _answers = [...answers.value, answer.value];
    _answers.sort((a, b) => b.date - a.date);
    while (_answers.length > 10) _answers.pop();
    answers.value = _answers;

    await writeText(answerText);
  } catch (err) {
    answer.value = null;
    onError(err);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="h-screen overflow-x-hidden overflow-y-auto">
    <div class="flex h-2/5 flex-col gap-4 p-4">
      <Textarea
        v-model="message"
        label="Prompt"
        label-class="text-secondary-foreground/70 h-[calc(100%-50px)]"
      />
      <div class="flex h-[50px] justify-center gap-2">
        <Button :disabled @click="fix">Fix</Button>
      </div>
    </div>

    <div
      v-if="answer"
      class="flex h-3/5 flex-col overflow-x-hidden overflow-y-auto px-2 pb-4 select-text"
    >
      <p>{{ answer.text }}</p>
    </div>
  </div>
</template>
