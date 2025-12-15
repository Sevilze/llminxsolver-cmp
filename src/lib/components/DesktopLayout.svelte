<script lang="ts">
  import MegaminxViewer from "./MegaminxViewer.svelte";
  import ControlPanel from "./ControlPanel.svelte";
  import SolutionsPanel from "./SolutionsPanel.svelte";
  import ScoredSolutionsPanel from "./ScoredSolutionsPanel.svelte";
  import StatusBar from "./StatusBar.svelte";
  import type {
    MegaminxState,
    SolverConfig,
    SolverState,
    IgnoreFlags,
    AllowedFacesMode,
    MetricType,
  } from "$lib/types/megaminx";

  interface Props {
    megaminxState: MegaminxState;
    config: SolverConfig;
    solverState: SolverState;
    onReset: () => void;
    onSwapCorners: (i: number, j: number) => void;
    onRotateCorner: (index: number, direction: 1 | -1) => void;
    onSwapEdges: (i: number, j: number) => void;
    onFlipEdge: (index: number) => void;
    onAllowedFacesChange: (mode: AllowedFacesMode) => void;
    onMetricChange: (metric: MetricType) => void;
    onLimitDepthChange: (limit: boolean) => void;
    onMaxDepthChange: (depth: number) => void;
    onIgnoreFlagChange: (flag: keyof IgnoreFlags, value: boolean) => void;
    onSolve: () => void;
    onCancel: () => void;
  }

  let {
    megaminxState,
    config,
    solverState,
    onReset,
    onSwapCorners,
    onRotateCorner,
    onSwapEdges,
    onFlipEdge,
    onAllowedFacesChange,
    onMetricChange,
    onLimitDepthChange,
    onMaxDepthChange,
    onIgnoreFlagChange,
    onSolve,
    onCancel,
  }: Props = $props();
</script>

<div class="flex flex-col h-full">
  <header
    class="flex items-center justify-between px-6 py-4 border-b border-border bg-secondary/30"
  >
    <div class="flex items-center gap-3">
      <div
        class="w-10 h-10 rounded-xl bg-megaminx-yellow flex items-center justify-center shadow-lg"
      >
        <svg
          viewBox="0 0 24 24"
          class="w-6 h-6 text-background"
          fill="currentColor"
        >
          <polygon
            points="12,2 22,8.5 22,15.5 12,22 2,15.5 2,8.5"
            stroke="currentColor"
            stroke-width="1.5"
            fill="none"
          />
          <circle cx="12" cy="12" r="4" fill="currentColor" />
        </svg>
      </div>
      <div>
        <h1 class="text-xl font-bold text-foreground">LLMinx Solver</h1>
        <p class="text-xs text-muted-foreground">
          Last Layer Megaminx Algorithm Finder
        </p>
      </div>
    </div>
    <div class="text-xs text-muted-foreground">v1.0</div>
  </header>

  <main class="flex-1 flex gap-4 p-4 overflow-hidden">
    <section class="flex flex-col gap-4 w-[320px] shrink-0">
      <div class="rounded-xl bg-secondary/50 p-4 border border-border">
        <h2 class="text-sm font-semibold text-foreground/80 mb-3">
          Starting Position
        </h2>
        <MegaminxViewer
          puzzleState={megaminxState}
          ignoreFlags={config.ignoreFlags}
          {onSwapCorners}
          {onRotateCorner}
          {onSwapEdges}
          {onFlipEdge}
          disabled={solverState.isSearching}
        />
      </div>
    </section>

    <section class="flex flex-col gap-4 w-60 shrink-0">
      <ControlPanel
        {config}
        isSearching={solverState.isSearching}
        {onAllowedFacesChange}
        {onMetricChange}
        {onLimitDepthChange}
        {onMaxDepthChange}
        {onIgnoreFlagChange}
        {onReset}
        {onSolve}
        {onCancel}
      />
    </section>

    <section class="flex-1 flex flex-col gap-3 min-w-0">
      <div class="flex-1 overflow-hidden">
        <ScoredSolutionsPanel
          metricLabel={config.metric === "FTM" ? "FTM" : "QTM"}
        />
      </div>
      <SolutionsPanel {solverState} defaultCollapsed={true} />
    </section>
  </main>

  <StatusBar {solverState} />
</div>
