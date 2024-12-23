<script lang="ts">
  import { cn } from '$lib/utils';
  import { Switch } from '$lib/components/ui/switch';
  import { Label } from '$lib/components/ui/label';

  interface Props {
    checked: boolean;
    label: string;
    labelClass?: string;
  }

  let { label, labelClass, checked = $bindable() }: Props = $props();
</script>

<Label class="flex items-center gap-2">
  <Switch bind:checked />
  <span class={cn('select-none font-normal', labelClass)}>{label}</span>
</Label>
