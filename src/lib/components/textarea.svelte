<script lang="ts">
  import { cn } from '$lib/utils';
  import { Textarea } from '$lib/components/ui/textarea';
  import { Label } from '$lib/components/ui/label';

  interface Props {
    class?: string;
    label: string;
    labelClass?: string;
    value?: string | null;
  }

  let { class: className, label, labelClass, value = $bindable() }: Props = $props();
</script>

<Label class={cn('flex flex-col gap-1', className)}>
  <span class={cn('select-none font-normal', labelClass)}>{label}</span>
  <Textarea bind:value class="h-full resize-none" />
</Label>
