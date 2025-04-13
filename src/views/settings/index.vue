<script setup lang="ts">
import { Textarea } from '@tb-dev/vue';
import { useSettingsStore } from '@/stores/settings';

const settings = useSettingsStore();
</script>

<template>
  <div class="size-full p-4">
    <Textarea
      v-model.trim="settings.token"
      label="Token"
      label-class="text-secondary-foreground/70 h-36"
    />
  </div>
</template>
