<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';
import { Label, Textarea } from '@tb-dev/vue-components';

const settings = useSettingsStore();
</script>

<template>
  <div class="h-screen overflow-x-hidden overflow-y-auto p-4">
    <Label>
      <span class="text-secondary-foreground/70">Token</span>
      <Textarea v-model="settings.token" class="min-h-28 w-[calc(100%-var(--sidebar-width))]" />
    </Label>
  </div>
</template>
