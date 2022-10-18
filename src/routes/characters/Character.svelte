<script lang="ts">
  import { getAvatar } from "$lib/util";
  import { createEventDispatcher } from "svelte";
  export let character: string;

  const dispatcher = createEventDispatcher();

  function handleClick() {
    dispatcher("choice", {character});
  }
</script>

<style lang="scss">
    .char-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 12px 8px;
    gap: 12px;
    width: fill;


    background: #1F1F1F;
    border: 1px solid #141414;
    border-radius: 4px;
  }
</style>

<div class="char-container" style="cursor:pointer" on:click={handleClick}>
  <img alt="Character Avatar" src={getAvatar(character)}>
  <p>{character}</p>
</div>
