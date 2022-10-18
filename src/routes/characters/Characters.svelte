<script lang="ts">
  import { getAvatar } from "$lib/util";
  import Character from "./Character.svelte";
  export let characters: string[];

  function handleClick() {
    
  }
</script>

<style lang="scss">
  .chars-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(96px + 24px, 1fr));
    grid-auto-rows: auto;

    gap: 12px 12px;
  }
</style>

<div class="chars-container">
  {#each characters as character}
    <Character {character} on:choice />
  {/each}
</div>
