<!-- Character selection screen -->
<script lang="ts">
  import { getCharacters, getOnlineFriends } from "$lib/rust";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { startSession } from "$lib/rust";
  import Characters from "./Characters.svelte";
  import { goto } from "$app/navigation";
  import { browser } from "$app/environment";
  import CharacterIcon, { ICON_SMALL } from "$lib/CharacterIcon.svelte";

  // Remember the recent characters too
  const recentCharacters = writable<string[]>(browser ? JSON.parse(window.localStorage.getItem("characters")??"[]") : []);
  if (browser)
    recentCharacters.subscribe((v) => {
      window.localStorage.setItem("characters", JSON.stringify(v));
    }) 

  // Characters list can't be gotten from page.ts
  // Svelte doesn't provide the "window" which causes Tauri to explode.
  let characters: string[] = [];
  let friends: string[] = [];

  onMount(async () => {
    characters = await getCharacters();
    friends = await getOnlineFriends();
  })

  async function chooseCharacter(event: CustomEvent<{character:string}>) {
    let newRecent = $recentCharacters.filter((v) => v!==event.detail.character);
    if (newRecent.unshift(event.detail.character) > 6) newRecent.pop();
    $recentCharacters = newRecent;
    await startSession(event.detail.character);
    await goto("/app-main", {replaceState: true, state: []});
  }
</script>

<style lang="scss">
  #characters-container {
    position: absolute;
    overflow: clip scroll;
    left: max(min(calc(50% - 340px), 20%), 0px);
    right: max(min(calc(50% - 340px), 20%), 0px);
    bottom: 128px;
    top: 64px;

    scrollbar-width: thin;

    &::-webkit-scrollbar {
      position: absolute;
      width: 8px;

      &-track {
        background: #ffffff00;
        &:hover {
          background: var(--color-gray-11);
        }
      }

      &-thumb {
        background: var(--color-gray-11);
        border-radius: 8px;

        &:hover {
          background: var(--color-gray-12);
        }
      }
    }
  }

  #recent {
    overflow: hidden;
  }

  #divider {
    left: 32px;
    right: 32px;
    height: 2px;
    border-radius: 2px;
    margin: 12px 24px;

    background: rgba(67, 67, 67, 0.4);
    align-self: stretch;
  }

  #friends-container {
    position: absolute;
    display: flex;
    flex-direction: column;
    
    bottom: 0px;
    left: 0px;
    right: 0px;

    padding: 16px 24px;
    gap: 8px;

    background-color: var(--color-gray-11);

    h3 {
      user-select: none;

      font-size: 16px;
      font-style: normal;
      font-weight: 500;
      line-height: 19px;
      margin: 0px;

      color: rgba(255, 255, 255, 0.87);
    }

    #friends {
      display: flex;
      flex-direction: row;
      gap: 8px;

      * {
        width: 32px;
        height: 32px;
      }
    }
  }
</style>

<div id="characters-container">
  {#if $recentCharacters.length > 0}
    <div id="recent">
      <Characters characters={$recentCharacters} on:choice={chooseCharacter}/>
    </div>
    <div id="divider"></div>
  {/if}
  <Characters characters={characters} on:choice={chooseCharacter}/>
</div>
{#if friends.length > 0 }
  <div id="friends-container">
    <h3>Online Friends</h3>
    <div id="friends">
      <!--Friends go in here-->
      {#each friends as friend}
        <CharacterIcon character={friend} {...ICON_SMALL}/>
      {/each}
    </div>
  </div>
{/if}
