<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    title: string;
    children: any;
  }

  let { isOpen, onClose, title, children }: Props = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") onClose();
  }

  onMount(() => {
    document.addEventListener("keydown", handleKeydown);
    return () => document.removeEventListener("keydown", handleKeydown);
  });
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-title"
  >
    <div
      class="fixed inset-0 bg-black/50 transition-opacity duration-200"
      onclick={onClose}
      onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") onClose(); }}
      role="button"
      tabindex="-1"
      aria-label="Close modal"
    ></div>

    <div class="relative bg-white rounded-xl shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-auto transform transition-all duration-200 scale-100">
      <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
        <h2 id="modal-title" class="text-lg font-semibold text-gray-800">{title}</h2>
        <button
          type="button"
          onclick={onClose}
          class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors"
          aria-label="Close"
        >
          <span aria-hidden="true">x</span>
        </button>
      </div>
      <div class="p-6">
        {@render children()}
      </div>
    </div>
  </div>
{/if}
