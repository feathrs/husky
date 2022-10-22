<script lang="ts">
  import { bookmarks, characters } from "$lib/data";
  import Character from "../Character.svelte";

  $: bookmarksWithStatus = $bookmarks.map((v) => {return { character: v, status: $characters[v]?.status??"offline", gender: $characters[v]?.gender??"none" }});
  $: groupOnlineBookmarks = bookmarksWithStatus.filter((v) => v.status !== "offline");
  $: groupOfflineBookmarks = bookmarksWithStatus.filter((v) => v.status === "offline");
</script>

<style lang="scss">
  .character-list {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    flex: auto;
    gap: 12px;
  }

  h3 {
    margin-bottom: 4px;

    &:first-child {
      margin-top: 0px;
    }
  }
</style>

<h3>Online</h3>
<div id="online" class="character-list">
{#each groupOnlineBookmarks as bookmark}
  <Character {...bookmark}/>
{/each}
</div>
<h3>Offline</h3>
<div id="offline" class="character-list">
  {#each groupOfflineBookmarks as bookmark}
  <Character {...bookmark}/>
  {/each}
</div>
