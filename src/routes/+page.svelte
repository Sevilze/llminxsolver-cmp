<script lang="ts">
  import DesktopLayout from "$lib/components/DesktopLayout.svelte";
  import MobileLayout from "$lib/components/MobileLayout.svelte";
  import {
    megaminxState,
    config,
    solverState,
    reset,
    swapCorners,
    rotateCorner,
    swapEdges,
    flipEdge,
    setAllowedFaces,
    setMetric,
    setLimitDepth,
    setMaxDepth,
    setIgnoreFlag,
    startSolve,
    cancelSolve,
  } from "$lib/stores/megaminx";

  let innerWidth = $state(768);
  let isDesktop = $derived(innerWidth >= 768);
</script>

<svelte:window bind:innerWidth />

{#if isDesktop}
  <DesktopLayout
    megaminxState={$megaminxState}
    config={$config}
    solverState={$solverState}
    onReset={reset}
    onSwapCorners={swapCorners}
    onRotateCorner={rotateCorner}
    onSwapEdges={swapEdges}
    onFlipEdge={flipEdge}
    onAllowedFacesChange={setAllowedFaces}
    onMetricChange={setMetric}
    onLimitDepthChange={setLimitDepth}
    onMaxDepthChange={setMaxDepth}
    onIgnoreFlagChange={setIgnoreFlag}
    onSolve={startSolve}
    onCancel={cancelSolve}
  />
{:else}
  <MobileLayout
    megaminxState={$megaminxState}
    config={$config}
    solverState={$solverState}
    onReset={reset}
    onSwapCorners={swapCorners}
    onRotateCorner={rotateCorner}
    onSwapEdges={swapEdges}
    onFlipEdge={flipEdge}
    onAllowedFacesChange={setAllowedFaces}
    onMetricChange={setMetric}
    onLimitDepthChange={setLimitDepth}
    onMaxDepthChange={setMaxDepth}
    onIgnoreFlagChange={setIgnoreFlag}
    onSolve={startSolve}
    onCancel={cancelSolve}
  />
{/if}
