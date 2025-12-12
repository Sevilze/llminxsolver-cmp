<script lang="ts">
  import type { IgnoreFlags } from "$lib/types/megaminx";

  interface Props {
    flags: IgnoreFlags;
    onChange: (flag: keyof IgnoreFlags, value: boolean) => void;
    disabled?: boolean;
  }

  let { flags, onChange, disabled = false }: Props = $props();

  const OPTIONS: { key: keyof IgnoreFlags; label: string }[] = [
    { key: "cornerPositions", label: "Corner positions" },
    { key: "edgePositions", label: "Edge positions" },
    { key: "cornerOrientations", label: "Corner orientations" },
    { key: "edgeOrientations", label: "Edge orientations" },
  ];

  function handleChange(key: keyof IgnoreFlags, e: Event) {
    const target = e.target as HTMLInputElement;
    onChange(key, target.checked);
  }
</script>

<div class="rounded-lg bg-secondary p-3 border border-border">
  <h3 class="text-sm font-medium text-foreground mb-2">Ignore</h3>
  <div class="space-y-2">
    {#each OPTIONS as option}
      <label class="flex items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          checked={flags[option.key]}
          onchange={(e) => handleChange(option.key, e)}
          {disabled}
          class="w-4 h-4 rounded border-input bg-background text-primary focus:ring-ring focus:ring-offset-0 disabled:opacity-50"
        />
        <span class="text-sm text-muted-foreground">{option.label}</span>
      </label>
    {/each}
  </div>
</div>
