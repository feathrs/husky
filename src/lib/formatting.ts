// Node tags:
// [b][i][u][s][color=][sup][sub][url=][user][icon][eicon][spoiler][noparse]
// Simple combinatorial nodes:
// - b
// - i
// - u
// - s
// - sup
// - sub
// - spoiler (untested in f-chat-proper)
// - noparse*
// Node types which can't have any other formatting combination (external) or nesting (internal)
// - eicon
// - icon
// - user
// Node types with attributes
// - url
// - color
// Additional considerations:
// - url, which has broken formatting combinations, and cannot be nested with itself
// - color, which overrides the inner color in nesting
// - noparse has no internal combinations, for obvious reasons

// Node types which can't have any other formatting combination
type ExclusiveRichNode = { eicon: string } | { character: string } | { character_icon: string }
type CombiningRichNode = {
  b?: boolean,
  i?: boolean,
  u?: boolean,
  s?: boolean,
  sup?: boolean,
  sub?: boolean,
  spoiler?: boolean,
  noparse?: boolean, // This probably shouldn't actually appear, unless I want to apply code-formatting.
  url?: string,
  color?: "red" | "orange" | "yellow" | "green" | "cyan" | "blue" | "purple" | "pink" | "black" | "brown" | "white" | "gray"
}

// We can make a nice linear parser, because it's just a big FSM (with nested states)
// And the state-nesting is just a stack, because all tags are optimistically balanced.
// We can either capture:
// - The opening tag, for BBCode
const tagOpening = /\[([a-z]+)(=[^\]]+)?\]/
// - The closing tag, for BBCode, including the quick-closing [/] tag I'm adding
const tagClosing = /\[\/([a-z]*)]\]/
// - The noparse closing tag, for convenience in differentiating it when closing
const noparseClosing = /\[\/(noparse)?\]/
// - Markdown-like balanced formatting tags (Specifically, Discord-style; underline, strikethrough, bold/bold-italic, italic, emote (eicon), spoiler)
const markdownTag = /__|~~|\*{2,3}|[_*]|:|\|\|/
// Then, we can parse everything by:
// Checking for an opening tag, and adding it onto the formatting stack
// Checking for a closing tag, and searching down from the top of the stack for the same tag-type. 
// - If found, remove all elements above it.
// - Else, it's potentially malformed, and we can treat it as standard inner-text
// For markdown we do the same, but we assume it's a closing tag first, and pessimistically stop our search if we reach BBCode in the stack


// Parse BBCode into RichText, optionally including Markdown.
export function parseBBCode(input: string, useMarkdown: boolean): string { return input }
