<script lang="ts">
  import { importRecipeFromUrl } from "$lib/tauri";
  import { toastStore } from "$lib/stores";
  import type { Recipe } from "$lib/types";

  interface Props {
    onSuccess: (recipe: Recipe) => void;
    onCancel: () => void;
  }

  let { onSuccess, onCancel }: Props = $props();
  let url = $state("");
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  async function handleSubmit() {
    const trimmedUrl = url.trim();
    if (!trimmedUrl) {
      error = "Please enter a URL";
      return;
    }

    error = null;
    isLoading = true;

    try {
      const recipe = await importRecipeFromUrl(trimmedUrl);
      toastStore.success(`Imported "${recipe.name}"`);
      onSuccess(recipe);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      error = message;
      // Don't show toast - error is already displayed inline in the modal
    } finally {
      isLoading = false;
    }
  }

  function handleCancel() {
    if (!isLoading) {
      onCancel();
    }
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
        disabled={isLoading}
        required
        placeholder="https://www.example.com/recipe/..."
        class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 disabled:bg-gray-100 disabled:cursor-not-allowed"
      />
      {#if error}
        <p class="mt-2 text-sm text-red-600">{error}</p>
      {/if}
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
        onclick={handleCancel}
        disabled={isLoading}
        class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Cancel
      </button>
      <button
        type="submit"
        disabled={isLoading || !url.trim()}
        class="px-6 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isLoading ? "Importing..." : "Import Recipe"}
      </button>
    </div>
  </form>
</div>
