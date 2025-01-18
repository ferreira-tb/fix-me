<script lang="ts">
  import '../app.css';
  import { commands } from '$lib/api';
  import { ModeWatcher } from 'mode-watcher';
  import { onMount, type Snippet } from 'svelte';
  import { settings } from '$lib/stores/settings';
  import { exit } from '@tauri-apps/plugin-process';
  import { Sidebar } from '$lib/components/sidebar';
  import { prompt } from '$lib/stores/prompt.svelte';
  import { history } from '$lib/stores/history.svelte';

  const { children }: { children: Snippet } = $props();

  function onKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') void exit(0);
  }

  onMount(() => {
    // prettier-ignore
    void settings.start()
      .then(() => prompt.start())
      .then(() => history.start())
      .then(() => commands.createTrayIcon())
      .then(() => commands.showWindow());
  });
</script>

<svelte:window onkeydown={onKeyDown} />

<Sidebar>
  <main class="h-screen w-full">
    <ModeWatcher defaultMode="dark" />
    {@render children()}
  </main>
</Sidebar>
