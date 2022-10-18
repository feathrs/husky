export function getAvatar(character: string): string {
  return `https://static.f-list.net/images/avatar/${encodeURI(character.toLowerCase())}.png`
}
