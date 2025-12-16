<script lang="ts">
  import Modal from "./Modal.svelte";

  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    destructive?: boolean;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    destructive = false,
    onConfirm,
    onCancel,
  }: Props = $props();
</script>

<Modal isOpen={open} {title} onClose={onCancel}>
  <p class="text-gray-600 mb-6">{message}</p>
  <div class="flex justify-end gap-3">
    <button
      onclick={onCancel}
      class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
    >
      {cancelLabel}
    </button>
    <button
      onclick={onConfirm}
      class="px-4 py-2 rounded-lg text-white"
      class:bg-red-600={destructive}
      class:hover:bg-red-700={destructive}
      class:bg-emerald-600={!destructive}
      class:hover:bg-emerald-700={!destructive}
    >
      {confirmLabel}
    </button>
  </div>
</Modal>
