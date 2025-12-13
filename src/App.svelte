<script lang="ts">
  import Greeting from "$lib/components/Greeting.svelte";
  import { greet } from "$lib/tauri";

  let name = $state("");
  let greeting = $state("");

  async function handleGreet() {
    greeting = await greet(name);
  }
</script>

<div class="min-h-screen bg-gray-50 flex flex-col items-center justify-center p-8">
  <h1 class="text-3xl font-bold text-blue-600 mb-8">feast</h1>

  <div class="bg-white rounded-lg shadow-md p-6 w-full max-w-md">
    <Greeting />

    <div class="mt-6 space-y-4">
      <input
        type="text"
        bind:value={name}
        placeholder="Enter your name"
        class="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
      <button
        type="button"
        onclick={handleGreet}
        class="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700 transition-colors"
      >
        Greet
      </button>
      {#if greeting}
        <p class="text-center text-gray-700 mt-4">{greeting}</p>
      {/if}
    </div>
  </div>
</div>
