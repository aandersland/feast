<script lang="ts">
  import { toastStore } from "$lib/stores/toast";
  import { fly } from "svelte/transition";
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
  {#each $toastStore as toast (toast.id)}
    <div
      transition:fly={{ x: 100, duration: 200 }}
      class="px-4 py-3 rounded-lg shadow-lg max-w-sm flex items-center gap-3"
      class:bg-emerald-600={toast.type === "success"}
      class:bg-red-600={toast.type === "error"}
      class:bg-blue-600={toast.type === "info"}
      class:text-white={true}
    >
      <span class="flex-1">{toast.message}</span>
      <button
        onclick={() => toastStore.remove(toast.id)}
        class="opacity-70 hover:opacity-100"
      >
        x
      </button>
    </div>
  {/each}
</div>
