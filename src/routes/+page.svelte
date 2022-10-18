<!--
  This is the root page -- This is practically a splash screen.
  Fetch everything out of the database, init anything in the background,
  and then push the user to login/account or login/character
-->

<script lang="ts">
  import {username, password, autoLogin} from "$lib/account";
  import {login} from "$lib/rust";
  import {goto} from "$app/navigation"
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

Root Page
<input name="username" placeholder="Username" bind:value={$username}>
<input name="password" type="password" placeholder="Password" bind:value={$password}>
<input type="checkbox" name="auto-login" id="auto-login" bind:value={$autoLogin}>
<label for="auto-login"> Auto-login </label>
<button name="login" on:click={doLogin}> Sign In </button>
