<script lang="ts">
  import type { SolverState } from "$lib/types/megaminx";

  interface Props {
    solverState: SolverState;
  }

  let { solverState }: Props = $props();

  let textAreaEl: HTMLDivElement | null = $state(null);
  let followMessages = $state(true);

  $effect(() => {
    if (followMessages && textAreaEl && solverState.solutions.length > 0) {
      textAreaEl.scrollTop = textAreaEl.scrollHeight;
    }
  });

  let solutionsText = $derived(
    solverState.solutions.length > 0
      ? solverState.solutions.join("\n")
      : solverState.isSearching
        ? "Searching for solutions..."
        : "Click 'Solve' to start searching for algorithms."
  );

  function handleFollowChange(e: Event) {
    const target = e.target as HTMLInputElement;
    followMessages = target.checked;
  }
</script>

<div
  class="flex flex-col rounded-lg bg-secondary border border-border overflow-hidden h-full"
>
  <div
    class="flex items-center justify-between px-3 py-2 border-b border-border"
  >
    <h3 class="text-sm font-medium text-foreground">Solving Status</h3>
    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="checkbox"
        checked={followMessages}
        onchange={handleFollowChange}
        class="w-3.5 h-3.5 rounded border-input bg-background text-primary focus:ring-ring focus:ring-offset-0"
      />
      <span class="text-xs text-muted-foreground">Follow messages</span>
    </label>
  </div>
  <div
    bind:this={textAreaEl}
    class="flex-1 min-h-[150px] max-h-[300px] overflow-y-auto p-3 font-mono text-xs leading-relaxed text-muted-foreground whitespace-pre-wrap select-text"
  >
    {solutionsText}
  </div>
</div>
