<script lang="ts">
  export let textRaw: string = "";
  export let textFormatted: string = "";

  let textbox: HTMLElement & ElementContentEditable;

  function generateFormatter(tag: string): () => void {
    const openTag = `[${tag}]`;
    const closeTag = `[/${tag}]`;
    return () => {
      let selection = window.getSelection();
      if (selection) {
        let start = selection.anchorOffset;
        let end = selection.focusOffset;
        if (end < start) {
          // Make sure that the start precedes the end
          let tmp = end;
          end = start;
          start = tmp;
        }
        let left = textRaw.slice(0, start);
        let mid = textRaw.slice(start, end);
        let right = textRaw.slice(end);
        textRaw = left + openTag + mid + closeTag + right;
        // I'm not sure how to retain the selection after this?
        // textbox.focus();
        // let range = document.createRange();
        // range.setStart(textbox, start);
        // range.setEnd(textbox, end + openTag.length + closeTag.length);
        // selection.addRange(range);
      } else {
        textRaw = textRaw + `${openTag}${closeTag}`;
      }
    };
  }
</script>

<div class="col" id="container">
  <div
    id="textbox"
    contenteditable="true"
    bind:this={textbox}
    bind:innerHTML={textFormatted}
    bind:textContent={textRaw}
  />
  <div id="formatting" class="row">
    <!-- Left side -->
    <img
      src="/fa/italic.svg"
      id="btn-italic"
      class:clickable={true}
      alt="italic"
      on:click={generateFormatter("i")}
    />
    <img
      src="/fa/bold.svg"
      id="btn-bold"
      class:clickable={true}
      alt="bold"
      on:click={generateFormatter("b")}
    />
    <img
      src="/fa/underline.svg"
      id="btn-underline"
      class:clickable={true}
      alt="underline"
      on:click={generateFormatter("u")}
    />
    <img
      src="/fa/strikethrough.svg"
      id="btn-strikethrough"
      class:clickable={true}
      alt="strikethrough"
      on:click={generateFormatter("s")}
    />
    <img
      src="/fa/subscript.svg"
      id="btn-subscript"
      class:clickable={true}
      alt="subscript"
      on:click={generateFormatter("sub")}
    />
    <img
      src="/fa/superscript.svg"
      id="btn-superscript"
      class:clickable={true}
      alt="superscript"
      on:click={generateFormatter("sup")}
    />
    <img
      src="/fa/eye-dropper.svg"
      id="btn-color"
      class:clickable={true}
      alt="color"
    />
    <img
      src="/fa/eye-slash.svg"
      id="btn-spoiler"
      class:clickable={true}
      alt="spoiler"
      on:click={generateFormatter("spoiler")}
    />
    <img
      src="/fa/text-slash.svg"
      id="btn-noparse"
      class:clickable={true}
      alt="noparse"
      on:click={generateFormatter("noparse")}
    />
    <div class="spreader" />
    <!-- Right side -->
    <img
      src="/fa/eye.svg"
      id="btn-preview"
      class:clickable={true}
      alt="preview"
    />
    <img src="/fa/dice.svg" id="btn-roll" class:clickable={true} alt="dice" />
    <img
      src="/fa/markdown.svg"
      id="btn-markdown"
      class:clickable={true}
      alt="markdown"
    />
  </div>
</div>

<style lang="scss">
  #container {
    flex: none;

    width: 100%;
    border-top: 1px solid #141414;
    padding: 6px 12px 12px;
    gap: 10px;
  }

  #textbox {
    align-items: center;
    padding: 8px;
    width: 100%;
    height: fit-content;

    background: #1f1f1f;
    border-radius: 4px;

    font-size: 12px;
    line-height: 15px;
  }

  #formatting img {
    width: 12px;
    height: 12px;
    margin: 4px 6px;
  }
</style>
