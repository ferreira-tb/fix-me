<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { handleError } from '@/lib/error';
import { onCtrlKeyDown } from '@tb-dev/vue';
import { commands } from '@/lib/api/bindings';
import { usePromptStore } from '@/stores/prompt';
import { useHistoryStore } from '@/stores/history';
import { useSettingsStore } from '@/stores/settings';
import { Button, Label, Textarea } from '@tb-dev/vue-components';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { computed, nextTick, onMounted, ref, useTemplateRef } from 'vue';

const promptStore = usePromptStore();
const { message, answer } = storeToRefs(promptStore);

const historyStore = useHistoryStore();
const { answers } = storeToRefs(historyStore);

const settings = useSettingsStore();

const messageEl = useTemplateRef('message-textarea');

const loading = ref(false);
const disabled = computed(() => {
  return loading.value || !message.value || !settings.token;
});

onCtrlKeyDown(['a', 'A'], (e) => {
  e.preventDefault();
  messageEl.value?.$el.focus();
  messageEl.value?.$el.select();
});

onMounted(async () => {
  await nextTick();
  messageEl.value?.$el.focus();
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
  }
  catch (err) {
    answer.value = null;
    handleError(err);
  }
  finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="h-screen overflow-x-hidden overflow-y-auto">
    <div class="flex h-2/5 flex-col gap-4 p-4">
      <Label class="h-[calc(100%-30px)]">
        <Textarea
          ref="message-textarea"
          v-model="message"
          class="h-full resize-none!"
          autocapitalize="off"
          autocomplete="off"
          autocorrect="off"
          autofocus
          spellcheck="false"
        />
      </Label>
      <div class="flex h-[50px] justify-center gap-2">
        <Button :disabled @click="fix">Fix</Button>
      </div>
    </div>

    <div
      v-if="answer"
      class="flex h-3/5 flex-col overflow-x-hidden overflow-y-auto px-2 pb-4 select-text"
    >
      <pre class="mt-4 whitespace-pre-wrap break-keep">{{ answer.text }}</pre>
    </div>
  </div>
</template>
