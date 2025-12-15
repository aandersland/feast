<script lang="ts">
  interface Props {
    onImport: (url: string) => void;
    onCancel: () => void;
  }

  let { onImport, onCancel }: Props = $props();
  let url = $state("");
  let isLoading = $state(false);

  function handleSubmit() {
    if (!url.trim()) return;
    isLoading = true;
    // Simulate import delay (actual parsing will be backend)
    setTimeout(() => {
      onImport(url);
      isLoading = false;
    }, 1500);
  }
</script>

<div>
  <p class="text-gray-500 mb-6">
    Paste a link to a recipe from any website. We'll extract the ingredients, instructions, and more.
  </p>

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Recipe URL</label>
      <input
        type="url"
        bind:value={url}
        required
        placeholder="https://www.example.com/recipe/..."
        class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      />
    </div>

    {#if isLoading}
      <div class="flex items-center justify-center py-8">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-600"></div>
        <span class="ml-3 text-gray-600">Importing recipe...</span>
      </div>
    {/if}

    <div class="flex justify-end gap-3 pt-4">
      <button
        type="button"
        onclick={onCancel}
        class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
      >
        Cancel
      </button>
      <button
        type="submit"
        disabled={isLoading}
        class="px-6 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors disabled:opacity-50"
      >
        Import Recipe
      </button>
    </div>
  </form>
</div>
