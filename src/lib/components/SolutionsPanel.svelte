<script lang="ts">
  import type { SolverState } from "$lib/types/megaminx";

  interface Props {
    solverState: SolverState;
    defaultCollapsed?: boolean;
  }

  let { solverState, defaultCollapsed = true }: Props = $props();

  let textAreaEl: HTMLDivElement | null = $state(null);
  let followMessages = $state(true);
  let isCollapsed = $state(true);

  $effect(() => {
    isCollapsed = defaultCollapsed;
  });

  $effect(() => {
    if (
      followMessages &&
      textAreaEl &&
      solverState.solutions.length > 0 &&
      !isCollapsed
    ) {
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

  function toggleCollapsed() {
    isCollapsed = !isCollapsed;
  }
</script>

<div
  class="flex flex-col rounded-lg bg-secondary border border-border overflow-hidden {isCollapsed
    ? ''
    : 'flex-1'}"
>
  <button
    type="button"
    onclick={toggleCollapsed}
    class="flex items-center justify-between px-3 py-2 border-b border-border hover:bg-background/30 transition-colors cursor-pointer w-full text-left"
  >
    <div class="flex items-center gap-2">
      <svg
        class="w-4 h-4 text-muted-foreground transition-transform {isCollapsed
          ? ''
          : 'rotate-90'}"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 5l7 7-7 7"
        />
      </svg>
      <h3 class="text-sm font-medium text-foreground">Raw Solutions</h3>
      {#if solverState.solutions.length > 0}
        <span class="text-xs text-muted-foreground"
          >({solverState.solutions.length})</span
        >
      {/if}
    </div>
    {#if !isCollapsed}
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <label
        class="flex items-center gap-2 cursor-pointer"
        onclick={(e) => e.stopPropagation()}
      >
        <input
          type="checkbox"
          checked={followMessages}
          onchange={handleFollowChange}
          class="w-3.5 h-3.5 rounded border-input bg-background text-primary focus:ring-ring focus:ring-offset-0"
        />
        <span class="text-xs text-muted-foreground">Follow</span>
      </label>
    {/if}
  </button>

  {#if !isCollapsed}
    <div
      bind:this={textAreaEl}
      class="flex-1 min-h-[100px] max-h-[200px] overflow-y-auto p-3 font-mono text-xs leading-relaxed text-muted-foreground whitespace-pre-wrap select-text"
    >
      {solutionsText}
    </div>
  {/if}
</div>
