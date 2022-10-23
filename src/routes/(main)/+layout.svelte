<script lang="ts">
  import Sidebar from "./Sidebar.svelte";
  import type { LayoutData } from "./$types";

  export let data: LayoutData;
</script>
<style lang="scss">
  #main-app {
    display: grid;
    grid-template-columns: 200px 1fr;
    grid-template-areas: "sidebar main";

    gap:0px;
    padding:0px;
  }
  #main {
    grid-area: main;
    max-height: 100vh;
    overflow: hidden;
    overflow-x: clip;
  }
</style>
<div id="main-app">
  <Sidebar {...data}/>
  <div id="main">
    <slot></slot>
  </div>
</div>
