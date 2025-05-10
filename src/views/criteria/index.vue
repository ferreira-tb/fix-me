<script setup lang="ts">
import { Textarea } from '@tb-dev/vue-components';
import { useSettingsStore } from '@/stores/settings';

const settings = useSettingsStore();
</script>

<template>
  <div class="flex h-screen flex-col gap-4 overflow-x-hidden overflow-y-auto p-4">
    <Textarea
      v-model="settings.grammar.message"
      label="Grammar"
      label-class="text-secondary-foreground/70 h-36"
    />

    <Textarea
      v-model="settings.readability.message"
      label="Readability"
      label-class="text-secondary-foreground/70 h-36"
    />

    <Textarea
      v-model="settings.tone.message"
      label="Tone"
      label-class="text-secondary-foreground/70 h-36"
    />

    <Textarea
      v-model="settings.politeness.message"
      label="Politeness"
      label-class="text-secondary-foreground/70 h-36"
    />

    <Textarea
      v-model="settings.formality.message"
      label="Formality"
      label-class="text-secondary-foreground/70 h-36"
    />
  </div>
</template>
