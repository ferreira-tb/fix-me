<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import Textarea from '$lib/components/textarea.svelte';
</script>

<div class="h-full p-4">
  <Textarea
    bind:value={$settings.token}
    label="Token"
    class="h-36"
    labelClass="text-secondary-foreground/70"
  />
</div>
