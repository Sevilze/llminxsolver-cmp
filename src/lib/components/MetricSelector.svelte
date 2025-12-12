<script lang="ts">
  import type { MetricType } from "$lib/types/megaminx";

  interface Props {
    value: MetricType;
    onChange: (metric: MetricType) => void;
    disabled?: boolean;
  }

  let { value, onChange, disabled = false }: Props = $props();

  function handleChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    onChange(target.value as MetricType);
  }
</script>

<div class="rounded-lg bg-secondary p-3 border border-border">
  <h3 class="text-sm font-medium text-foreground mb-2">Metric</h3>
  <select
    {value}
    onchange={handleChange}
    {disabled}
    class="w-full bg-background text-foreground rounded-md px-3 py-2 text-sm border border-input focus:outline-none focus:ring-2 focus:ring-ring disabled:opacity-50 disabled:cursor-not-allowed"
  >
    <option value="FTM">Face Turn Metric</option>
    <option value="QTM">Fifth Turn Metric</option>
  </select>
</div>
