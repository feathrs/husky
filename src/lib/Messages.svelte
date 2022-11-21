<script lang="ts">
  import type { Message, MessageChannel } from "$lib/types";
  import { getMessages } from "$lib/rust";
  import { onMount } from "svelte";
  import CharacterIcon from "./CharacterIcon.svelte";

  export let channel: MessageChannel;

  let messages: Message[] = [];

  onMount(async () => {
    messages = await getMessages(channel);
  });

  function joinRolls(rolls: string[]): string {
    return rolls.join("+");
  }

  function format(content: string): string {
    // This shouldn't actually be in here.
    return content;
  }
</script>

{#each messages as message}
  <div class="message">
    <CharacterIcon character={message.character} />
    <div class="content message-{message.content.type}">
      <span class="character-name">{message.character}</span>
      {#if message.content.type == "roll"}
        <img src="/fa/dice.svg" class="dice" alt="dice" />
        <!-- I'd prefer to be able to syntax-highlight these, perhaps. Can use CSS:after.content to do joining. -->
        <span class="rolls">{joinRolls(message.content.content[0])}</span> =&gt;
        <span class="result">{message.content.content[2]}</span>
      {:else if message.content.type == "bottle"}
        <img src="/fa/dice.svg" class="dice" alt="dice" />
        bottle =&gt;
        <span class="formatting-character">{message.content.content}</span>
      {:else if message.content.type == "emote"}
        <img src="/fa/asterisk.svg" class="emote" alt="asterisk" />
        <i>{@html format(message.content.content)}</i>
      {:else}
        {@html format(message.content.content)}
      {/if}
    </div>
  </div>
{/each}

<style lang="scss">
</style>
