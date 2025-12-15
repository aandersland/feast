<script lang="ts">
  type ViewMode = "category" | "recipe";

  interface Props {
    activeView: ViewMode;
    onViewChange: (view: ViewMode) => void;
  }

  let { activeView, onViewChange }: Props = $props();
</script>

<div class="flex items-center gap-2 text-sm">
  <span class="text-gray-500">View by:</span>
  <div class="flex rounded-lg border border-gray-200 overflow-hidden">
    <button
      type="button"
      onclick={() => onViewChange("category")}
      class="px-3 py-1.5 transition-colors
        {activeView === 'category' ? 'bg-emerald-100 text-emerald-700' : 'hover:bg-gray-50'}"
    >
      Category
    </button>
    <button
      type="button"
      onclick={() => onViewChange("recipe")}
      class="px-3 py-1.5 transition-colors
        {activeView === 'recipe' ? 'bg-emerald-100 text-emerald-700' : 'hover:bg-gray-50'}"
    >
      Recipe
    </button>
  </div>
</div>
