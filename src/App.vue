<script setup lang="ts">
import { onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { capitalCase } from 'change-case';
import { useColorMode } from '@vueuse/core';
import { ChevronUp } from 'lucide-vue-next';
import { commands } from '@/lib/api/bindings';
import { usePromptStore } from '@/stores/prompt';
import { exit } from '@tauri-apps/plugin-process';
import { useHistoryStore } from '@/stores/history';
import { useSettingsStore } from '@/stores/settings';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  handleError,
  onKeyDown,
  Sidebar,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  Switch,
} from '@tb-dev/vue';

const promptStore = usePromptStore();
const historyStore = useHistoryStore();
const settings = useSettingsStore();

const route = useRoute();

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  storageKey: 'fix-me:color-mode',
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(() => {
  // prettier-ignore
  promptStore.$tauri.start()
    .then(() => historyStore.$tauri.start())
    .then(() => settings.$tauri.start())
    .then(() => commands.createTrayIcon())
    .then(() => commands.showWindow())
    .err()
});
</script>

<template>
  <Sidebar default-open width="200px">
    <main class="h-screen w-[calc(100vw-var(--sidebar-width))] select-none">
      <div class="size-full overflow-hidden p-0">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <component :is="Component" />
          </template>
        </RouterView>
      </div>
    </main>

    <template #content>
      <SidebarGroup>
        <SidebarGroupLabel>Fixes</SidebarGroupLabel>
        <SidebarGroupContent>
          <div class="flex w-full flex-col gap-3">
            <Switch v-model="settings.grammar.enabled" label="Grammar" />
            <Switch v-model="settings.readability.enabled" label="Readability" />
            <Switch v-model="settings.tone.enabled" label="Tone" />
          </div>
        </SidebarGroupContent>
      </SidebarGroup>

      <SidebarGroup>
        <SidebarGroupLabel>Style</SidebarGroupLabel>
        <SidebarGroupContent>
          <div class="flex w-full flex-col gap-3">
            <Switch v-model="settings.politeness.enabled" label="Polite" />
            <Switch v-model="settings.formality.enabled" label="Formal" />
          </div>
        </SidebarGroupContent>
      </SidebarGroup>
    </template>

    <template #footer>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button v-if="typeof route.name === 'string'" variant="outline">
            <span>{{ capitalCase(route.name) }}</span>
            <ChevronUp class="ml-auto" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent side="top" class="w-[var(--reka-dropdown-menu-trigger-width)]">
          <DropdownMenuItem>
            <RouterLink to="/" class="w-full">Chat</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/criteria" class="w-full">Criteria</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/history" class="w-full">History</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/settings" class="w-full">Settings</RouterLink>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </template>
  </Sidebar>
</template>
