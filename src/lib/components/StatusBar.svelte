<script lang="ts">
  import type { SolverState } from "$lib/types/megaminx";

  interface Props {
    solverState: SolverState;
  }

  let { solverState }: Props = $props();
</script>

<div
  class="flex items-center gap-4 px-4 py-2 bg-secondary/50 border-t border-border"
>
  <span class="text-sm text-muted-foreground flex-1 truncate">
    {solverState.status}
  </span>
  {#if solverState.isSearching}
    <div class="flex items-center gap-2 min-w-[150px]">
      <div class="flex-1 h-2 bg-muted rounded-full overflow-hidden">
        <div
          class="h-full bg-primary transition-all duration-300"
          style="width: {solverState.progress * 100}%"
        ></div>
      </div>
      <span class="text-xs text-muted-foreground w-12 text-right">
        {Math.round(solverState.progress * 100)}%
      </span>
    </div>
  {/if}
</div>
