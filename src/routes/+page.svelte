<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import Textarea from '$lib/components/textarea.svelte';
  import { settings } from '$lib/stores/settings';
  import { prompt } from '$lib/stores/prompt';
  import { commands } from '$lib/api';

  let loading = $state(false);
  const disabled = $derived.by(() => {
    return loading || !$prompt.message || !$settings.token;
  });

  async function fix() {
    try {
      loading = true;
      const answer = await commands.prompt($prompt.message);
      $prompt.answer = { text: answer, date: Date.now() };
    } finally {
      loading = false;
    }
  }
</script>

<div class="size-full">
  <div class="flex h-2/5 flex-col gap-4 p-4">
    <Textarea
      bind:value={$prompt.message}
      label="Prompt"
      class="h-full"
      labelClass="text-secondary-foreground/70"
    />
    <div class="flex justify-center gap-2">
      <Button {disabled} onclick={fix}>Fix</Button>
    </div>
  </div>

  <div class="flex h-3/5 flex-col overflow-y-auto overflow-x-hidden px-2 pb-4">
    {#if $prompt.answer}
      <p>{$prompt.answer.text}</p>
    {/if}
  </div>
</div>
