<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { initWasm } from "$lib/wasm";

  interface Props {
    children: import("svelte").Snippet;
  }

  let { children }: Props = $props();
  let wasmReady = $state(false);

  onMount(async () => {
    await initWasm();
    wasmReady = true;
  });
</script>

{#if wasmReady}
  <div class="h-screen w-screen overflow-hidden">
    {@render children()}
  </div>
{:else}
  <div class="h-screen w-screen flex items-center justify-center bg-background">
    <div class="text-muted-foreground">Loading...</div>
  </div>
{/if}
