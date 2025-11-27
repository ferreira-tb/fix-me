<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { capitalCase } from 'change-case';
import { useColorMode } from '@vueuse/core';
import { ChevronUp } from 'lucide-vue-next';
import { commands } from '@/lib/api/bindings';
import { exit } from '@tauri-apps/plugin-process';
import { useSettingsStore } from '@/stores/settings';
import { handleError, onKeyDown } from '@tb-dev/vue';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  Label,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarInset,
  SidebarProvider,
  Switch,
} from '@tb-dev/vue-components';

const route = useRoute();
const isSidebarOpen = ref(true);
const settings = useSettingsStore();

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  storageKey: 'fix-me:color-mode',
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(async () => {
  try {
    await commands.showWindow();
  }
  catch (err) {
    handleError(err);
  }
});
</script>

<template>
  <SidebarProvider
    v-model:open="isSidebarOpen"
    class="overflow-x-hidden overflow-y-auto"
    style="--sidebar-width: 200px"
  >
    <Sidebar variant="inset" class="p-0">
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Fixes</SidebarGroupLabel>
          <SidebarGroupContent>
            <div class="flex w-full flex-col gap-3">
              <Label>
                <Switch v-model="settings.grammar.enabled" />
                <span>Grammar</span>
              </Label>
              <Label>
                <Switch v-model="settings.readability.enabled" />
                <span>Readability</span>
              </Label>
              <Label>
                <Switch v-model="settings.tone.enabled" />
                <span>Tone</span>
              </Label>
            </div>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Style</SidebarGroupLabel>
          <SidebarGroupContent>
            <div class="flex w-full flex-col gap-3">
              <Label>
                <Switch v-model="settings.politeness.enabled" />
                <span>Polite</span>
              </Label>
              <Label>
                <Switch v-model="settings.formality.enabled" />
                <span>Formal</span>
              </Label>
            </div>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter>
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button v-if="typeof route.name === 'string'" variant="outline">
              <span>{{ capitalCase(route.name) }}</span>
              <ChevronUp class="ml-auto" />
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent side="top" class="w-(--reka-dropdown-menu-trigger-width)">
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
      </SidebarFooter>
    </Sidebar>

    <SidebarInset class="m-0! rounded-none! p-0">
      <div class="size-full overflow-hidden p-0">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <component :is="Component" />
          </template>
        </RouterView>
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
