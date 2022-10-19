<!--
  This is the root page -- This is practically a splash screen.
  Fetch everything out of the database, init anything in the background,
  and then push the user to login/account or login/character
-->

<script lang="ts">
  import { username, password, autoLogin } from "$lib/account";
  import { login } from "$lib/rust";
  import { goto } from "$app/navigation"
  import { onMount } from "svelte";

  onMount(async () => {
    if ($autoLogin) {
      // I promise that if autoLogin is set, the credentials are set.
      await doLogin();
    }
  })

  async function doLogin() {
    await login($username!, $password!);
    goto("/characters");
  }
</script>

<style lang="scss">
  #main {
    position: absolute;
    justify-content: center;
    align-items: center;
    width: 256px;
    height: 100%;
    left: calc(50% - (256px/2));
    gap: 12px;

    h1 {
      font-size: 48px;
      font-weight: 300;
    }

    input {
      box-sizing: border-box;
      background: var(--color-gray-11);
      border: 1px solid var(--color-gray-12);
      border-radius: 4px;

      padding: 8px 12px;
      width: 100%;

      &[type="checkbox"] {
        height: 16px;
        width: 16px;
      }
    }

    #final-row {
      width: 100%;
      justify-content: end;
      align-content: center;
      align-items: center;
      gap: 4px;

      button {
        padding: 6px 12px;
        box-sizing: border-box;
        background: rgba(67, 67, 67, 0.4);
        border: 1px solid var(--color-gray-9);
        border-radius: 4px;
        margin-left: 16px;
      }
    }
  }

  :root {
    background-color: var(--color-gray-10);
  }  
</style>

<div id="main" class="col">
  <h1>Husky</h1>
  <input name="username" placeholder="Username" bind:value={$username}>
  <input name="password" type="password" placeholder="Password" bind:value={$password}>
  <div id="final-row" class="row">
    <input type="checkbox" name="auto-login" id="auto-login" bind:checked={$autoLogin}>
    <label for="auto-login"> Auto-login </label>
    <button name="login" on:click={doLogin}> Sign In </button>
  </div>
</div>
