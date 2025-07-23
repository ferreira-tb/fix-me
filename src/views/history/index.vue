<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { onError } from '@/lib/utils';
import { useHistoryStore } from '@/stores/history';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Button, Card, CardContent, CardFooter } from '@tb-dev/vue-components';

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
      <Card v-for="answer of answers" :key="answer.date" :title="date(answer.date)" class="p-0">
        <CardContent class="px-4 pt-4 pb-0">
          <p>{{ answer.text }}</p>
        </CardContent>

        <CardFooter class="px-4 pt-2 pb-4">
          <div class="flex w-full items-center justify-end gap-4">
            <Button size="sm" @click="() => copy(answer.text)">Copy</Button>
            <Button variant="destructive" size="sm" @click="() => remove(answer.date)">
              Remove
            </Button>
          </div>
        </CardFooter>
      </Card>
    </div>
  </div>
</template>
