<script lang="ts">
  import "../app.css";

  interface Props {
    children: import("svelte").Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="h-screen w-screen overflow-hidden">
  {@render children()}
</div>
