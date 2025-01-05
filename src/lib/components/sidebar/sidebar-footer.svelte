<script lang="ts">
  import { page } from '$app/state';
  import { ChevronUp } from 'lucide-svelte';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

  const route = $derived.by(() => page.route.id);
  const routeName = $derived.by(() => {
    switch (route) {
      case '/settings':
        return 'Settings';
      case '/':
      default:
        return 'Chat';
    }
  });
</script>

<Sidebar.Footer>
  <Sidebar.Menu>
    <Sidebar.MenuItem>
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <Sidebar.MenuButton
              {...props}
              class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              {routeName}
              <ChevronUp class="ml-auto" />
            </Sidebar.MenuButton>
          {/snippet}
        </DropdownMenu.Trigger>
        <DropdownMenu.Content side="top" class="w-[--bits-dropdown-menu-anchor-width]">
          <DropdownMenu.Item>
            <a href="/" class="w-full">Chat</a>
          </DropdownMenu.Item>
          <DropdownMenu.Item>
            <a href="/settings" class="w-full">Settings</a>
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </Sidebar.MenuItem>
  </Sidebar.Menu>
</Sidebar.Footer>
