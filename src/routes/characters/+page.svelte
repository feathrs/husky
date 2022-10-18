<!-- Character selection screen -->
<script lang="ts">
  import { getCharacters } from "$lib/rust";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { startSession } from "$lib/rust";
  import Characters from "./Characters.svelte";
  import { goto } from "$app/navigation";
  import { browser } from "$app/environment";

  // Remember the recent characters too
  const recentCharacters = writable<string[]>(browser ? JSON.parse(window.localStorage.getItem("characters")??"[]") : []);
  if (browser)
    recentCharacters.subscribe((v) => {
      window.localStorage.setItem("characters", JSON.stringify(v));
    }) 

  // Characters list can't be gotten from page.ts
  // Svelte doesn't provide the "window" which causes Tauri to explode.
  let characters: string[] = [];

  onMount(async () => {
    characters = await getCharacters();
  })

  async function chooseCharacter(event: CustomEvent<{character:string}>) {
    let newRecent = $recentCharacters.filter((v) => v!==event.detail.character);
    if (newRecent.unshift(event.detail.character) > 6) newRecent.pop();
    $recentCharacters = newRecent;
    await startSession(event.detail.character);
    await goto("/app-main", {replaceState: true, state: [""]});
  }
</script>

Characters Page<br>
{#if $recentCharacters.length > 0}
  Recent characters
  <Characters characters={$recentCharacters} on:choice={chooseCharacter}/>
{/if}
All characters
<Characters characters={characters} on:choice={chooseCharacter}/>
