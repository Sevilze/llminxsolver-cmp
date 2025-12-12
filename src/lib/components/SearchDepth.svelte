<script lang="ts">
  interface Props {
    limitDepth: boolean;
    maxDepth: number;
    onLimitChange: (limit: boolean) => void;
    onDepthChange: (depth: number) => void;
    disabled?: boolean;
  }

  let { limitDepth, maxDepth, onLimitChange, onDepthChange, disabled = false }: Props = $props();

  function handleLimitChange(e: Event) {
    const target = e.target as HTMLInputElement;
    onLimitChange(target.checked);
  }

  function handleDepthChange(e: Event) {
    const target = e.target as HTMLInputElement;
    onDepthChange(parseInt(target.value) || 1);
  }
</script>

<div class="rounded-lg bg-secondary p-3 border border-border">
  <h3 class="text-sm font-medium text-foreground mb-2">Search Depth</h3>
  <div class="flex items-center gap-3">
    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="checkbox"
        checked={limitDepth}
        onchange={handleLimitChange}
        {disabled}
        class="w-4 h-4 rounded border-input bg-background text-primary focus:ring-ring focus:ring-offset-0 disabled:opacity-50"
      />
      <span class="text-sm text-muted-foreground">Limit</span>
    </label>
    <input
      type="number"
      min={1}
      max={50}
      value={maxDepth}
      onchange={handleDepthChange}
      disabled={disabled || !limitDepth}
      class="flex-1 bg-background text-foreground rounded-md px-3 py-1.5 text-sm border border-input focus:outline-none focus:ring-2 focus:ring-ring disabled:opacity-50 disabled:cursor-not-allowed"
    />
  </div>
</div>
