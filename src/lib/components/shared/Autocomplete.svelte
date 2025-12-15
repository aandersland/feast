<script lang="ts">
  interface Props {
    options: string[];
    value: string;
    onSelect: (value: string) => void;
    onClear: () => void;
    placeholder?: string;
  }

  let { options, value, onSelect, onClear, placeholder = "Search..." }: Props = $props();

  let query = $state("");
  let isOpen = $state(false);
  let inputRef: HTMLInputElement;

  let filteredOptions = $derived(
    query.length > 0
      ? options.filter((o) =>
          o.toLowerCase().includes(query.toLowerCase()) && o !== value
        ).slice(0, 8)
      : []
  );

  function handleSelect(option: string) {
    onSelect(option);
    query = "";
    isOpen = false;
  }

  function handleClear() {
    onClear();
    query = "";
  }

  function handleInputFocus() {
    isOpen = true;
  }

  function handleInputBlur() {
    // Delay to allow click on option
    setTimeout(() => { isOpen = false; }, 150);
  }
</script>

<div class="relative">
  {#if value}
    <div class="flex items-center gap-2 px-3 py-2 bg-emerald-50 border border-emerald-200 rounded-lg">
      <span class="text-emerald-700 capitalize">{value}</span>
      <button
        type="button"
        onclick={handleClear}
        class="text-emerald-500 hover:text-emerald-700"
        aria-label="Clear filter"
      >
        x
      </button>
    </div>
  {:else}
    <input
      bind:this={inputRef}
      type="text"
      bind:value={query}
      onfocus={handleInputFocus}
      onblur={handleInputBlur}
      {placeholder}
      class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
    />
    {#if isOpen && filteredOptions.length > 0}
      <ul class="absolute z-10 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg max-h-48 overflow-auto">
        {#each filteredOptions as option}
          <li>
            <button
              type="button"
              onclick={() => handleSelect(option)}
              class="w-full text-left px-3 py-2 hover:bg-emerald-50 capitalize"
            >
              {option}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
</div>
