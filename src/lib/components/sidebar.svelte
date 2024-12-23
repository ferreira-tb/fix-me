<script lang="ts">
  import Switch from './switch.svelte';
  import { settings } from '$lib/stores/settings';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import SidebarFooter from './sidebar-footer.svelte';
</script>

<Sidebar.Root>
  <Sidebar.Content class="select-none">
    <Sidebar.Group>
      <Sidebar.GroupLabel>Fixes</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <div class="flex flex-col gap-3">
          <Switch label="Grammar" bind:checked={$settings.grammar} />
          <Switch label="Readability" bind:checked={$settings.readability} />
          <Switch label="Tone" bind:checked={$settings.tone} />
        </div>
      </Sidebar.GroupContent>
    </Sidebar.Group>

    <Sidebar.Group>
      <Sidebar.GroupLabel>Style</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <div class="flex flex-col gap-3">
          <Switch label="Polite" bind:checked={$settings.politeness} />
          <Switch label="Formal" bind:checked={$settings.formality} />
        </div>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>

  <SidebarFooter />
</Sidebar.Root>
