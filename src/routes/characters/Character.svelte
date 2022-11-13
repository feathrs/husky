<script lang="ts">
  import { getAvatar } from "$lib/util";
  import { createEventDispatcher } from "svelte";
  export let character: string;

  const dispatcher = createEventDispatcher();

  function handleClick() {
    dispatcher("choice", { character });
  }
</script>

<div class="char-container" on:click={handleClick}>
  <img alt="Character Avatar" src={getAvatar(character)} />
  <p>{character}</p>
</div>

<style lang="scss">
  .char-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 12px 8px;
    gap: 12px;
    width: fill;
    overflow-wrap: anywhere;

    background: #1f1f1f;
    border: 1px solid #141414;
    border-radius: 4px;

    cursor: pointer;
  }

  * {
    user-select: none;
  }
</style>
