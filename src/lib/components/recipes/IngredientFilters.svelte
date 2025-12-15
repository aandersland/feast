<script lang="ts">
  import { allIngredients } from "$lib/stores";
  import Autocomplete from "$lib/components/shared/Autocomplete.svelte";

  interface Props {
    filters: [string, string, string];
    onFiltersChange: (filters: [string, string, string]) => void;
  }

  let { filters, onFiltersChange }: Props = $props();

  function updateFilter(index: number, value: string) {
    const newFilters = [...filters] as [string, string, string];
    newFilters[index] = value;
    onFiltersChange(newFilters);
  }

  function clearFilter(index: number) {
    updateFilter(index, "");
  }

  // Exclude already-selected ingredients from options
  let availableIngredients = $derived(
    $allIngredients.filter((i) => !filters.includes(i))
  );
</script>

<div class="flex flex-wrap gap-3">
  <span class="text-sm text-gray-500 self-center">Filter by ingredient:</span>
  {#each [0, 1, 2] as index}
    <div class="w-44">
      <Autocomplete
        options={availableIngredients}
        value={filters[index]}
        onSelect={(v) => updateFilter(index, v)}
        onClear={() => clearFilter(index)}
        placeholder="Add ingredient..."
      />
    </div>
  {/each}
</div>
