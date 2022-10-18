<script>
  export let mainCharacter = "Alex Reaper";
</script>
<style lang="scss">
  #sidebar {
    box-sizing: border-box;

    grid-area: sidebar;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0px;
    gap: 8px;
    width: 100%;
    height: 100vh;

    background-color: var(--color-gray-11);
  }

  #character-header {
    box-sizing: border-box;

    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 12px;
    gap: 8px;

    width: 100%;

    background: var(--color-gray-12);
    border-bottom: 1px solid var(--color-gray-10);

    order: 0;
  }

  #sidebar-main {
    display: flex;
    flex-direction: column;
    align-items: left;
    gap: 8px;

    width: 100%;
    background-color: var(--color-gray-11);

    flex: none;
    flex-grow: 1;
    order: 1;
  }

  #people {
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0px;

    background: var(--color-gray-10);
    border: 1px solid var(--color-gray-9);
    border-radius: 4px;

    margin: 0px 12px;

    img {
      filter: invert(100%);
      height: 24px;
      width: 24px;
      margin: 6px 12px;

      flex: none;
      order: 0;
      flex-grow: 0;
    }

    p {
      margin: 0px;
      user-select: none;

      flex: none;
      order: 1;
      flex-grow: 1;
    }
  }

  #private-messages {

  }

  #channels {

  }

  #sidebar-footer {
    box-sizing: border-box;
    display: grid;
    grid-template-columns: repeat(5, 1fr);

    align-items: center;
    padding: 10px 12px;

    width: 100%;

    background: var(--color-gray-12);
    border-top: 1px solid var(--color-gray-10);

    order: 2;

    img {
      filter: invert(100%);
      width: 16px;
      height: 16px;
    }

    div {
      display: flex;
      justify-content: center;
      align-items: center;
    }
  }

  #main-character {
    width: 56px;
    height: 56px;
  }

  #header-col {
    display: flex;
    flex-direction: column;
    align-items: left;

    a {
      text-align: left;
      align-self: stretch;
      color: rgba(255, 255, 255, 0.97);
    }
  }

  #alt-characters {
    display: flex;
    flex-direction: row;
    gap: 8px;
    align-self: stretch;

    flex: none;
    order: 1;
    align-self: stretch;
    flex-grow: 0;
  }

  .alt-character {
    width: 32px;
    height: 32px;

    border-radius: 2px;
  }

  #alt-character-button {
    box-sizing: border-box;
    width: 32px;
    height: 32px;

    border: 2px dashed rgba(255, 255, 255, 0.6);
    border-radius: 2px;

    img {
      filter: invert(100%);
      opacity: 0.6;
      position: relative;

      width: 22px;
      height: 22px;

      left: calc(50% - 11px);
      top: calc(50% - 11px);
    }
  }
</style>

<div id="sidebar">
  <div id="character-header">
    <div id="main-character" style="order:0;background-color:#434343;border-radius:4px;"></div> <!-- This will stand in for the moment -->
    <div id="header-col">
      <!-- This is just so that we can stack the name and alt profiles -->
      <a href="_blank">{mainCharacter}</a>
      <div id="alt-characters">
        <div class="alt-character" style="background-color:#434343"></div>
        <div class="alt-character" style="background-color:#434343"></div>
        <div id="alt-character-button">
          <img src="/fa/plus.svg" alt="add">
        </div>
      </div>
    </div>
  </div>
  <div id="sidebar-main">
    <div id="people">
      <img src="/fa/user.svg" alt="person">
      <p>People</p>
    </div>
    <div id="private-messages"></div>
    <div id="channels"></div>
  </div>
  <div id="sidebar-footer">
    <div id="settings">
      <img src="/fa/gear.svg" alt="Settings">
    </div>
    <div id="ads">
      <img src="/fa/rectangle-ad.svg" alt="Ads">
    </div>
    <div id="logs">
      <img src="/fa/file-lines.svg" alt="Logs">
    </div>
    <div id="console">
      <img src="/fa/terminal.svg" alt="Console">
    </div>
    <div id="sign-out">
      <img src="/fa/right-from-bracket.svg" alt="Sign-Out">
    </div>
  </div>
</div>
