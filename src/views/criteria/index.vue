<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';
import { Label, Textarea } from '@tb-dev/vue-components';

const settings = useSettingsStore();
</script>

<template>
  <div class="flex h-screen flex-col gap-4 overflow-x-hidden overflow-y-auto p-4">
    <Label class="h-36">
      <span class="text-secondary-foreground/70">Grammar</span>
      <Textarea v-model="settings.grammar.message" />
    </Label>

    <Label class="h-36">
      <span class="text-secondary-foreground/70">Readability</span>
      <Textarea v-model="settings.readability.message" />
    </Label>

    <Label class="h-36">
      <span class="text-secondary-foreground/70">Tone</span>
      <Textarea v-model="settings.tone.message" />
    </Label>

    <Label class="h-36">
      <span class="text-secondary-foreground/70">Politeness</span>
      <Textarea v-model="settings.politeness.message" />
    </Label>

    <Label class="h-36">
      <span class="text-secondary-foreground/70">Formality</span>
      <Textarea v-model="settings.formality.message" />
    </Label>
  </div>
</template>
