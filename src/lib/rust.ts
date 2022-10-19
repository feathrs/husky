// This is where I'm packing all of the functions for talking to Tauri backend
import { invoke } from "@tauri-apps/api/tauri";
import { sessions } from "$lib/session";
import type { Channel, ChannelData, Character, CharacterData } from "$lib/types";


export type RTCharacter = {
  name: string,
  status: string
}

export type MessageTarget = { character: Character } | { channel: Channel };

export function login(username: string, password: string) {
  return invoke("login", { username, password });
}

export async function startSession(character: string) {
  await invoke("start_session", { character: character });
  sessions.update((v) => {
    v.unshift(character);
    return v;
  })
}

export function getOwnCharacters(): Promise<string[]> {
  return invoke("get_own_characters", {});
}

export function getFriends(): Promise<Character[]> {
  return invoke("get_friends", {});
}

export function getBookmarks(): Promise<Character[]> {
  return invoke("get_bookmarks", {});
}

export function getCharacter(character: Character): Promise<CharacterData> {
  return invoke("get_character", { character });
}

export function getChannel(channel: Channel): Promise<ChannelData> {
  return invoke("get_channel", { channel });
}

export function getAllCharacters(): Promise<Record<Character, CharacterData>> {
  return invoke("get_all_characters");
}

// Session functions (all take session)
export async function sendMessage(session: Character, target: MessageTarget, message: string) {
  await invoke("session_send_message", { session, target, message });
}

export async function sendDice(session: Character, target: MessageTarget, dice: string) {
  await invoke("session_send_dice", { session, target, dice });
}

export async function joinChannel(session: Character, channel: Channel) {
  await invoke("session_join_channel", { session, channel });
}
