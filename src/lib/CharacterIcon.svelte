<script lang="ts" context="module">
  export const ICON_SMALL = {
    iconSize: 32,
    statusSize: 8
  }

  export const ICON_LARGE = {
    iconSize: 56,
    statusSize: 12
  }
</script>

<script lang="ts">
  import { getAvatar } from "./util";

  export let status = "online";
  export let character: string;
  export let iconSize = 32;
  export let statusSize = 8;
</script>

<style lang="scss">
  #container {
    display: block;
    width: var(--icon-size);
    height: var(--icon-size);

    line-height: 0px;

    img {
      border-radius: calc(var(--icon-size) / 8);

      width: 100%;
      height: 100%;
    }
  }

  #status {
    box-sizing: border-box;
    display: inline-block;

    position: relative;
    left: calc(100% - var(--status-size));
    top: calc(0px - var(--status-size));

    width: var(--status-size);
    height: var(--status-size);

    border-radius: calc(var(--status-size) / 2) 0px calc(var(--status-size) / 4) 0px;
    border-width: calc(var(--status-size) / 4) 0px 0px calc(var(--status-size) / 4);
    border-style: solid;
    border-color: var(--color-gray-11);

    &.online {
      background-color: #49aa19;
    }
    &.looking {
      background-color: #49aa19;
    }
    &.away {
      background-color: #d8bd14;
    }
    &.busy {
      background-color: #d87a16;
    }
    &.do_not_disturb {
      background-color: #d32029;
    }
    &.offline {
      background-color: #8c8c8c;
    }
  }
</style>

<div id="container" style="
  --icon-size: {iconSize}px;
  --status-size: {statusSize}px;
">
  <img src={getAvatar(character)} alt=character>
  <div id="status" class={status}></div>
</div>
