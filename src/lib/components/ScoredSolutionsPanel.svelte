<script lang="ts">
  import type { ScoredSolution } from "$lib/types/megaminx";
  import { scoredSolutions } from "$lib/stores/megaminx";

  interface Props {
    maxSolutions?: number;
    metricLabel?: string;
  }

  let { maxSolutions = 20, metricLabel = "Moves" }: Props = $props();

  let copiedIndex: number | null = $state(null);

  let displayedSolutions = $derived($scoredSolutions.slice(0, maxSolutions));

  async function copyToClipboard(algorithm: string, index: number) {
    try {
      await navigator.clipboard.writeText(algorithm);
      copiedIndex = index;
      setTimeout(() => {
        copiedIndex = null;
      }, 1500);
    } catch {
      console.error("Failed to copy algorithm");
    }
  }
</script>

<div
  class="flex flex-col rounded-lg bg-secondary border border-border overflow-hidden h-full"
>
  <div
    class="flex items-center justify-between px-3 py-2 border-b border-border"
  >
    <h3 class="text-sm font-medium text-foreground">
      Scored Algorithms
      {#if $scoredSolutions.length > 0}
        <span class="text-muted-foreground font-normal"
          >({Math.min(maxSolutions, $scoredSolutions.length)} of {$scoredSolutions.length})</span
        >
      {/if}
    </h3>
    <div class="flex items-center gap-2">
      <label class="flex items-center gap-1.5 text-xs text-muted-foreground">
        Top
        <input
          type="number"
          bind:value={maxSolutions}
          min="1"
          max="100"
          class="w-14 px-2 py-1 text-xs rounded border border-input bg-background text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
        />
      </label>
    </div>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#if $scoredSolutions.length === 0}
      <div
        class="flex items-center justify-center h-full p-4 text-sm text-muted-foreground"
      >
        No solutions to score yet. Run the solver to generate algorithms.
      </div>
    {:else}
      <table class="w-full text-xs">
        <thead class="sticky top-0 bg-secondary border-b border-border">
          <tr>
            <th
              class="px-3 py-2 text-left font-medium text-muted-foreground w-16"
              >MCC</th
            >
            <th
              class="px-3 py-2 text-left font-medium text-muted-foreground w-16"
              >{metricLabel}</th
            >
            <th class="px-3 py-2 text-left font-medium text-muted-foreground"
              >Algorithm</th
            >
          </tr>
        </thead>
        <tbody>
          {#each displayedSolutions as solution, index}
            <tr
              class="border-b border-border/50 hover:bg-background/50 cursor-pointer transition-colors group"
              onclick={() => copyToClipboard(solution.algorithm, index)}
              title="Click to copy"
            >
              <td class="px-3 py-2 text-muted-foreground tabular-nums">
                {solution.mcc.toFixed(1)}
              </td>
              <td class="px-3 py-2 text-muted-foreground tabular-nums">
                {solution.moveCount}
              </td>
              <td class="px-3 py-2 font-mono text-foreground">
                <div class="flex items-center justify-between">
                  <span class="truncate">{solution.algorithm}</span>
                  {#if copiedIndex === index}
                    <span class="text-green-500 text-[10px] ml-2 shrink-0"
                      >Copied!</span
                    >
                  {:else}
                    <span
                      class="text-muted-foreground/0 group-hover:text-muted-foreground/50 text-[10px] ml-2 shrink-0 transition-colors"
                    >
                      Click to copy
                    </span>
                  {/if}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>
