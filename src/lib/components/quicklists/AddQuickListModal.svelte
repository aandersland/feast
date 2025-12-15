<script lang="ts">
  import Modal from "../shared/Modal.svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onAdd: (name: string) => void;
  }

  let { isOpen, onClose, onAdd }: Props = $props();

  let listName = $state("");

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!listName.trim()) return;

    onAdd(listName.trim());
    listName = "";
    onClose();
  }
</script>

<Modal {isOpen} {onClose} title="Create New Quick List">
  {#snippet children()}
    <form onsubmit={handleSubmit}>
      <div class="mb-4">
        <label for="list-name" class="block text-sm font-medium text-gray-700 mb-1">
          List Name
        </label>
        <input
          id="list-name"
          type="text"
          bind:value={listName}
          placeholder="e.g., Weekly Essentials"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          autofocus
        />
      </div>
      <div class="flex justify-end gap-3">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg transition-colors"
        >
          Cancel
        </button>
        <button
          type="submit"
          class="px-4 py-2 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
        >
          Create List
        </button>
      </div>
    </form>
  {/snippet}
</Modal>
