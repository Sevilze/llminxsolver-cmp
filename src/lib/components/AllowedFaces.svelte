<script lang="ts">
  import type { AllowedFacesMode } from "$lib/types/megaminx";

  interface Props {
    value: AllowedFacesMode;
    onChange: (mode: AllowedFacesMode) => void;
    disabled?: boolean;
  }

  let { value, onChange, disabled = false }: Props = $props();

  const MODES: { value: AllowedFacesMode; label: string }[] = [
    { value: "R_U", label: "R, U" },
    { value: "R_U_L", label: "R, U, L" },
    { value: "R_U_F", label: "R, U, F" },
    { value: "R_U_L_F", label: "R, U, L, F" },
    { value: "R_U_L_F_B", label: "R, U, L, F, B" },
  ];

  function handleChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    onChange(target.value as AllowedFacesMode);
  }
</script>

<div class="rounded-lg bg-secondary p-3 border border-border">
  <h3 class="text-sm font-medium text-foreground mb-2">Allowed Faces</h3>
  <select
    {value}
    onchange={handleChange}
    {disabled}
    class="w-full bg-background text-foreground rounded-md px-3 py-2 text-sm border border-input focus:outline-none focus:ring-2 focus:ring-ring disabled:opacity-50 disabled:cursor-not-allowed"
  >
    {#each MODES as mode}
      <option value={mode.value}>{mode.label}</option>
    {/each}
  </select>
</div>
