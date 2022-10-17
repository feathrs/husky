<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
  
    let username = "";
    let password = "";
    let greetMsg = "";
    let characters: string[] = [];
    let chosenCharacter = "";
  
    async function login() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        await invoke("login", { username, password });
        greetMsg = "I think it logged in."
        characters = await invoke("get_characters", {});
    }

    async function spawnSession() {
        await invoke("start_session", { character: chosenCharacter });
    }
</script>
  
<div>
    <div class="row">
        <input id="username" placeholder="Username" bind:value={username} />
        <input id="password" placeholder="Password" bind:value={password} />
        <button on:click={login}> Login </button>
    </div>
    <p>{greetMsg}</p>
    <select id="character" bind:value={chosenCharacter}>
        {#each characters as character}
            <option value={character}>{character}</option>
        {/each}
    </select>
    <button on:click={spawnSession}>Connect session...</button>
</div>
  