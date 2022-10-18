<script>
  import Sidebar from "./Sidebar.svelte";

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
  }
</style>
<div id="main-app">
  <Sidebar/>
  <div id="main">
    <slot></slot>
  </div>
</div>
