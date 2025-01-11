<script lang="ts">
  import { onError } from '$lib/utils';
  import { history } from '$lib/stores/history';
  import * as Card from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  function date(time: number) {
    return new Date(time).toLocaleString();
  }

  function copy(text: string) {
    writeText(text).catch(onError);
  }

  function remove(time: number) {
    $history.answers = $history.answers.filter((answer) => answer.date !== time);
  }
</script>

<div class="h-screen overflow-y-auto overflow-x-hidden p-4">
  <div class="space-y-2">
    {#each $history.answers as answer (answer.date)}
      <Card.Root>
        <Card.Header class="px-4 pb-2 pt-4">
          <Card.Title>{date(answer.date)}</Card.Title>
        </Card.Header>
        <Card.Content class="px-4 py-2">
          <p>{answer.text}</p>
        </Card.Content>
        <Card.Footer class="flex items-center justify-end gap-4 px-4 pb-4 pt-2">
          <Button size="sm" onclick={() => copy(answer.text)}>Copy</Button>
          <Button variant="destructive" size="sm" onclick={() => remove(answer.date)}>
            Remove
          </Button>
        </Card.Footer>
      </Card.Root>
    {/each}
  </div>
</div>
