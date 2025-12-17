<script lang="ts">
  import { onMount } from "svelte";
  import { recipeStore, recipesLoading, groupRecipes, getRecipeProtein, getRecipeStarch } from "$lib/stores";
  import type { Recipe, RecipeViewMode } from "$lib/types";
  import RecipeCard from "./recipes/RecipeCard.svelte";
  import RecipeForm from "./recipes/RecipeForm.svelte";
  import ImportRecipe from "./recipes/ImportRecipe.svelte";
  import IngredientFilters from "./recipes/IngredientFilters.svelte";
  import RecipeViewToggle from "./recipes/RecipeViewToggle.svelte";
  import Modal from "./shared/Modal.svelte";
  import ConfirmDialog from "./shared/ConfirmDialog.svelte";
  import RecipeDetailPanel from "./recipes/RecipeDetailPanel.svelte";

  type ModalView = "none" | "create" | "import" | "edit";

  // Panel state
  let selectedRecipe = $state<Recipe | null>(null);
  let isPanelOpen = $derived(selectedRecipe !== null);

  // Modal state
  let modalView = $state<ModalView>("none");
  let editingRecipe = $state<Recipe | null>(null);

  // Filter state
  let searchQuery = $state("");
  let ingredientFilters = $state<[string, string, string]>(["", "", ""]);
  let viewMode = $state<RecipeViewMode>("default");

  // Delete confirmation state
  let deleteTarget: { id: string; name: string } | null = $state(null);

  onMount(() => {
    recipeStore.load();
  });

  async function handleDelete() {
    if (deleteTarget) {
      await recipeStore.remove(deleteTarget.id);
      deleteTarget = null;
    }
  }

  // Filtering logic
  let filteredRecipes = $derived.by(() => {
    let recipes = $recipeStore;

    // Text search
    if (searchQuery) {
      const q = searchQuery.toLowerCase();
      recipes = recipes.filter(
        (r) =>
          r.name.toLowerCase().includes(q) ||
          r.tags.some((t) => t.toLowerCase().includes(q))
      );
    }

    // Ingredient filters (AND logic)
    const activeFilters = ingredientFilters.filter((f) => f !== "");
    if (activeFilters.length > 0) {
      recipes = recipes.filter((r) => {
        const recipeIngredients = r.ingredients.map((i) => i.name.toLowerCase());
        return activeFilters.every((filter) =>
          recipeIngredients.some((ing) => ing.includes(filter))
        );
      });
    }

    return recipes;
  });

  // Grouping logic
  let groupedRecipes = $derived.by(() => {
    if (viewMode === "default") return null;

    if (viewMode === "tag") {
      return groupRecipes(filteredRecipes, (r) => r.tags[0] ?? null);
    }
    if (viewMode === "protein") {
      return groupRecipes(filteredRecipes, getRecipeProtein);
    }
    if (viewMode === "starch") {
      return groupRecipes(filteredRecipes, getRecipeStarch);
    }
    return null;
  });

  function selectRecipe(recipe: Recipe) {
    selectedRecipe = recipe;
  }

  function closePanel() {
    selectedRecipe = null;
  }

  function handleEdit(recipe: Recipe) {
    editingRecipe = recipe;
    modalView = "edit";
  }

  async function handleSaveRecipe(data: Omit<Recipe, "id" | "createdAt">) {
    try {
      if (editingRecipe) {
        const updated = await recipeStore.update(editingRecipe.id, data);
        // Update selected if editing current
        if (selectedRecipe?.id === editingRecipe.id && updated) {
          selectedRecipe = updated;
        }
      } else {
        await recipeStore.add(data);
      }
      modalView = "none";
      editingRecipe = null;
    } catch {
      // Error already handled by store with toast
    }
  }

  function handleImport(recipe: Recipe) {
    // Recipe already created by backend, just add to store
    recipeStore.load(); // Refresh to get the new recipe
    modalView = "none";
  }

  function closeModal() {
    modalView = "none";
    editingRecipe = null;
  }

  // Clear panel if selected recipe is deleted
  $effect(() => {
    const current = selectedRecipe;
    if (current && !$recipeStore.find(r => r.id === current.id)) {
      selectedRecipe = null;
    }
  });

  // Grid columns based on panel state
  let gridCols = $derived(isPanelOpen ? "grid-cols-1 lg:grid-cols-2" : "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4");
</script>

<div class="flex h-full">
  <!-- Recipe List -->
  <div class="flex-1 overflow-auto p-6 {isPanelOpen ? 'border-r border-gray-200' : ''}">
    <div class="max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6">
      <!-- Header -->
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-xl sm:text-2xl font-bold text-gray-800">Recipes</h1>
        <div class="flex gap-2">
          <button
            type="button"
            onclick={() => modalView = "import"}
            class="px-4 py-2 border border-emerald-600 text-emerald-600 rounded-lg hover:bg-emerald-50 transition-colors"
          >
            Import from URL
          </button>
          <button
            type="button"
            onclick={() => modalView = "create"}
            class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
          >
            + Add Recipe
          </button>
        </div>
      </div>

      <!-- Filters Row -->
      <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
        <div class="flex flex-wrap items-center gap-4">
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search recipes by name or tag..."
            class="w-64 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          />
          <IngredientFilters
            filters={ingredientFilters}
            onFiltersChange={(f) => ingredientFilters = f}
          />
        </div>
        <RecipeViewToggle
          activeView={viewMode}
          onViewChange={(v) => viewMode = v}
        />
      </div>

      <!-- Recipe Grid -->
      {#if $recipesLoading}
        <div class="flex items-center justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-600"></div>
          <span class="ml-3 text-gray-600">Loading recipes...</span>
        </div>
      {:else if $recipeStore.length === 0}
        <div class="text-center py-12">
          <p class="text-gray-500">No recipes yet. Add your first recipe to get started!</p>
        </div>
      {:else if groupedRecipes}
        <!-- Grouped View -->
        {#each [...groupedRecipes.entries()] as [group, recipes]}
          <div class="mb-8">
            <h2 class="text-lg font-semibold text-gray-700 mb-4 capitalize">{group}</h2>
            <div class="grid {gridCols} gap-4 sm:gap-6">
              {#each recipes as recipe}
                <RecipeCard
                  {recipe}
                  onSelect={selectRecipe}
                  isSelected={selectedRecipe?.id === recipe.id}
                />
              {/each}
            </div>
          </div>
        {/each}
      {:else}
        <!-- Default Grid View -->
        <div class="grid {gridCols} gap-4 sm:gap-6">
          {#each filteredRecipes as recipe}
            <RecipeCard
              {recipe}
              onSelect={selectRecipe}
              isSelected={selectedRecipe?.id === recipe.id}
            />
          {/each}
        </div>
      {/if}

      {#if !$recipesLoading && $recipeStore.length > 0 && filteredRecipes.length === 0}
        <div class="text-center py-12 text-gray-500">
          No recipes found. Try adjusting your filters or add a new recipe!
        </div>
      {/if}
    </div>
  </div>

  <!-- Detail Panel -->
  {#if isPanelOpen && selectedRecipe}
    <div class="w-[480px] flex-shrink-0 bg-white border-l border-gray-200">
      <RecipeDetailPanel
        recipe={selectedRecipe}
        onClose={closePanel}
        onEdit={() => handleEdit(selectedRecipe!)}
      />
    </div>
  {/if}
</div>

<!-- Modals -->
<Modal isOpen={modalView === "create"} onClose={closeModal} title="Add New Recipe">
  {#snippet children()}
    <RecipeForm onSave={handleSaveRecipe} onCancel={closeModal} />
  {/snippet}
</Modal>

<Modal isOpen={modalView === "edit" && editingRecipe !== null} onClose={closeModal} title="Edit Recipe">
  {#snippet children()}
    <RecipeForm
      recipe={editingRecipe}
      onSave={handleSaveRecipe}
      onCancel={closeModal}
    />
  {/snippet}
</Modal>

<Modal isOpen={modalView === "import"} onClose={closeModal} title="Import Recipe">
  {#snippet children()}
    <ImportRecipe onSuccess={handleImport} onCancel={closeModal} />
  {/snippet}
</Modal>

<ConfirmDialog
  open={deleteTarget !== null}
  title="Delete Recipe"
  message={`Are you sure you want to delete "${deleteTarget?.name}"? This cannot be undone.`}
  confirmLabel="Delete"
  destructive={true}
  onConfirm={handleDelete}
  onCancel={() => (deleteTarget = null)}
/>
