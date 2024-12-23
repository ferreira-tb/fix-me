<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { commands } from '$lib/api';
  import { ModeWatcher } from 'mode-watcher';
  import { settings } from '$lib/stores/settings';
  import { prompt } from '$lib/stores/prompt';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import AppSidebar from '$lib/components/sidebar.svelte';
  import { exit } from '@tauri-apps/plugin-process';

  let { children } = $props();

  function onKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') void exit(0);
  }

  onMount(() => {
    // prettier-ignore
    settings.start()
      .then(() => prompt.start())
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
