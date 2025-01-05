<script lang="ts">
  import '../app.css';
  import { commands } from '$lib/api';
  import { ModeWatcher } from 'mode-watcher';
  import { prompt } from '$lib/stores/prompt';
  import { onMount, type Snippet } from 'svelte';
  import { settings } from '$lib/stores/settings';
  import { exit } from '@tauri-apps/plugin-process';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import AppSidebar from '$lib/components/sidebar.svelte';

  const { children }: { children: Snippet } = $props();

  function onKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') void exit(0);
  }

  onMount(() => {
    // prettier-ignore
    void settings.start()
      .then(() => prompt.start())
      .then(() => commands.createTrayIcon())
      .then(() => commands.showWindow());
  });
</script>

<svelte:window onkeydown={onKeyDown} />

<Sidebar.Provider>
  <AppSidebar />
  <main class="h-screen w-full">
    <ModeWatcher defaultMode="dark" />
    {@render children()}
  </main>
</Sidebar.Provider>
