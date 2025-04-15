<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { onError } from '@/lib/utils';
import { Button, Card } from '@tb-dev/vue';
import { useHistoryStore } from '@/stores/history';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const historyStore = useHistoryStore();
const { answers } = storeToRefs(historyStore);

function date(time: number) {
  return new Date(time).toLocaleString();
}

function copy(text: string) {
  writeText(text).catch(onError);
}

function remove(time: number) {
  answers.value = answers.value.filter((answer) => answer.date !== time);
}
</script>

<template>
  <div class="h-screen overflow-x-hidden overflow-y-auto p-4">
    <div class="space-y-2">
      <Card
        v-for="answer of answers"
        :key="answer.date"
        :title="date(answer.date)"
        class="p-0"
        header-class="px-4 pb-2 pt-4"
        content-class="px-4 py-2"
        footer-class="px-0"
      >
        <template #footer>
          <div class="flex w-full items-center justify-end gap-4 px-4 pt-2 pb-4">
            <Button size="sm" @click="() => copy(answer.text)">Copy</Button>
            <Button variant="destructive" size="sm" @click="() => remove(answer.date)">
              Remove
            </Button>
          </div>
        </template>

        <p>{{ answer.text }}</p>
      </Card>
    </div>
  </div>
</template>
