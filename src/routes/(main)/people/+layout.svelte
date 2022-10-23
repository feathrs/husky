<!-- This is the default page, *and* it's the People screen -->
<script lang="ts">
  import type { LayoutData } from './$types';
  import { goto } from '$app/navigation';


  export let data: LayoutData;

  async function select(tab: string) {
    await goto(`/people/${tab}`)
  }
</script>

<style lang="scss">
  #root {
    display: grid;
    grid-template-rows: min-content 1fr;
    grid-template-areas: 
      "header" 
      "content";

    height: 100%;
  }

  #content {
    grid-area: "content";
    display: flex;
    flex-direction: column; 
    overflow: scroll;
    overflow-x: clip;
    padding: 12px;
    flex-grow: none;
    flex-basis: content;
    bottom: 0px;

    > * {
      flex: 0 0 auto;
    }

    *:first-child {
      margin-top: 0px;
    }

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

  #header {
    grid-area: "header";
    display: flex;
    flex-direction: column;

    padding: 8px 12px;
    gap: 12px;

    background: var(--color-gray-11);

    h2 {
      font-size: 24px;
      font-weight: 400;
      margin: 0px;
      margin-top: 2px;
    }
  }
  #button-row {
    display: flex;
    flex-direction: row;
    padding: 0px;
    gap: 8px;
    align-self: stretch;
  }
  .button {
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    align-items: center;
    
    font-size: 14px;
    line-height: normal;
    padding: 4px 8px;
    gap: 8px;

    background: var(--color-gray-10);
    border: 1px solid var(--color-gray-9);
    border-radius: 2px;

    &.selected {
      background: var(--color-gray-9);
    }

    img {
      width: 14px;
      height: 14px;
    }
  }
  .spreader {
    flex: 1;
  }
</style>

<div id="root">
  <div id="header">
    <h2>People</h2>
    <div id="button-row">
      <div class="clickable button" class:selected={data.currentTab==="everyone"} id="button-everyone" on:click={()=>select("everyone")}>
        <img src="/fa/circle-check.svg" alt="">
        Everyone
      </div>
      <div class="clickable button" class:selected={data.currentTab==="recent"} id="button-recent" on:click={()=>select("recent")}>
        <img src="/fa/clock.svg" alt="">
        Recent
      </div>
      <div class="clickable button" class:selected={data.currentTab==="friends"} id="button-friends" on:click={()=>select("friends")}>
        <img src="/fa/user-group-solid.svg" alt="">
        Friends
      </div>
      <div class="clickable button" class:selected={data.currentTab==="bookmarks"} id="button-bookmarks" on:click={()=>select("bookmarks")}>
        <img src="/fa/bookmark.svg" alt="">
        Bookmarks
      </div>
      <div class="spreader"></div>
      <div class="clickable button" class:selected={data.currentTab==="search"} id="button-search" on:click={()=>select("search")}>
        <img src="/fa/magnifying-glass.svg" alt="">
        Character Search
      </div>
    </div>
  </div>
  <div id="content"> 
    <slot/>
  </div>
</div>
