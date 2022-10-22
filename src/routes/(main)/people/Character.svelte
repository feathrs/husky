<script lang="ts">
  import CharacterIcon, { ICON_SMALL } from "$lib/CharacterIcon.svelte";
  import type { Character } from "$lib/types";

  export let character: Character;
  export let status: string = "offline";
  export let gender: string = "none";
</script>

<style lang="scss">
  #character-container {
    display: flex;
    flex-flow: row wrap;
    align-items: center;
    overflow: clip;
    flex: auto;

    padding: 0px;
    padding-right: 12px;
    gap: 8px;
    max-width: fit-content;

    border-radius: 4px;
    background: rgba(67, 67, 67, 0.4);
    border: 1px solid var(--color-gray-9);

    font-size: 14px;
    font-weight: 500;

    &.gender-{
      &none {
        color: #BFBFBF;
      }
      &Male {
        color: #3c9ae8;
      }
      &Male-herm {
        color: #2b4acb;
      }
      &Herm {
        color: #854eca;
      }
      &Shemale {
        color: #ab7ae0;
      }
      &Female {
        color: #e0529c;
      }
      &Cunt-boy {
        color: #8bbb11;
      }
      &Transgender {
        color: #d87a16;
      }
    }
  }
</style>

<div id="character-container" class="clickable gender-{gender}" on:click>
  <CharacterIcon {character} {status} {...ICON_SMALL}/>
  {character}
</div>
