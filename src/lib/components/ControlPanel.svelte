<script lang="ts">
  import type { SolverConfig, IgnoreFlags, AllowedFacesMode, MetricType } from "$lib/types/megaminx";
  import AllowedFaces from "./AllowedFaces.svelte";
  import MetricSelector from "./MetricSelector.svelte";
  import SearchDepth from "./SearchDepth.svelte";
  import IgnoreOptions from "./IgnoreOptions.svelte";

  interface Props {
    config: SolverConfig;
    isSearching: boolean;
    onAllowedFacesChange: (mode: AllowedFacesMode) => void;
    onMetricChange: (metric: MetricType) => void;
    onLimitDepthChange: (limit: boolean) => void;
    onMaxDepthChange: (depth: number) => void;
    onIgnoreFlagChange: (flag: keyof IgnoreFlags, value: boolean) => void;
    onReset: () => void;
    onSolve: () => void;
    onCancel: () => void;
  }

  let {
    config,
    isSearching,
    onAllowedFacesChange,
    onMetricChange,
    onLimitDepthChange,
    onMaxDepthChange,
    onIgnoreFlagChange,
    onReset,
    onSolve,
    onCancel,
  }: Props = $props();
</script>

<div class="flex flex-col gap-3">
  <AllowedFaces
    value={config.allowedFaces}
    onChange={onAllowedFacesChange}
    disabled={isSearching}
  />

  <MetricSelector
    value={config.metric}
    onChange={onMetricChange}
    disabled={isSearching}
  />

  <SearchDepth
    limitDepth={config.limitDepth}
    maxDepth={config.maxDepth}
    onLimitChange={onLimitDepthChange}
    onDepthChange={onMaxDepthChange}
    disabled={isSearching}
  />

  <IgnoreOptions
    flags={config.ignoreFlags}
    onChange={onIgnoreFlagChange}
    disabled={isSearching}
  />

  <div class="flex gap-2 pt-2">
    <button
      onclick={onReset}
      disabled={isSearching}
      class="flex-1 px-4 py-2 text-sm font-medium rounded-md bg-secondary text-foreground hover:bg-secondary/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors border border-border"
    >
      Reset
    </button>
    <button
      onclick={isSearching ? onCancel : onSolve}
      class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {isSearching
        ? 'bg-destructive hover:bg-destructive/80 text-destructive-foreground'
        : 'bg-primary hover:bg-primary/90 text-primary-foreground'}"
    >
      {isSearching ? "Cancel" : "Solve"}
    </button>
  </div>
</div>
