<script lang="ts">
  import type { RecipeViewMode } from "$lib/types";

  interface Props {
    activeView: RecipeViewMode;
    onViewChange: (view: RecipeViewMode) => void;
  }

  let { activeView, onViewChange }: Props = $props();

  const viewOptions: { value: RecipeViewMode; label: string }[] = [
    { value: "default", label: "All" },
    { value: "tag", label: "Category" },
    { value: "protein", label: "Protein" },
    { value: "starch", label: "Starch" },
  ];
</script>

<div class="flex items-center gap-2 text-sm">
  <span class="text-gray-500">View by:</span>
  <div class="flex rounded-lg border border-gray-200 overflow-hidden">
    {#each viewOptions as option}
      <button
        type="button"
        onclick={() => onViewChange(option.value)}
        class="px-3 py-1.5 transition-colors
          {activeView === option.value ? 'bg-emerald-100 text-emerald-700' : 'hover:bg-gray-50'}"
      >
        {option.label}
      </button>
    {/each}
  </div>
</div>
