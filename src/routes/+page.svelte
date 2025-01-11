<script lang="ts">
  import { commands } from '$lib/api';
  import { onError } from '$lib/utils';
  import { prompt } from '$lib/stores/prompt';
  import { history } from '$lib/stores/history';
  import { settings } from '$lib/stores/settings';
  import { Button } from '$lib/components/ui/button';
  import Textarea from '$lib/components/textarea.svelte';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  let loading = $state(false);
  const disabled = $derived.by(() => {
    return loading || !$prompt.message || !$settings.token;
  });

  async function fix() {
    try {
      loading = true;
      const answerText = await commands.prompt($prompt.message);
      const answer = { text: answerText, date: Date.now() };
      $prompt.answer = answer;

      const answers = [...$history.answers, answer];
      answers.sort((a, b) => b.date - a.date);
      while (answers.length > 10) answers.pop();
      $history.answers = answers;

      await writeText(answerText);
    } catch (err) {
      $prompt.answer = null;
      onError(err);
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
