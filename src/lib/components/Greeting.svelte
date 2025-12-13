<script lang="ts">
  import { appStore } from "$lib/stores";
</script>

<div class="text-center">
  <p class="text-gray-600">Welcome to</p>
  <p class="text-xl font-semibold text-gray-800">{$appStore.appName} v{$appStore.version}</p>
</div>
