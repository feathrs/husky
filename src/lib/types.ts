export type Character = string;
export type Channel = string;
export type ChannelData = {
  channelMode: string,
  members: Set<Character>,
  description: string,
  title: string,
}
export type CharacterData = {
  gender: string,
  status: string,
  statusMessage: string
}
