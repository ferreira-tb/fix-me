<script setup lang="ts">
import { Textarea } from '@tb-dev/vue-components';
import { useSettingsStore } from '@/stores/settings';

const settings = useSettingsStore();
</script>

<template>
  <div class="h-screen overflow-x-hidden overflow-y-auto p-4">
    <Textarea
      v-model="settings.token"
      label="Token"
      label-class="text-secondary-foreground/70 h-36"
    />
  </div>
</template>
