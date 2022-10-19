// This is where I'm packing all of the functions for talking to Tauri backend
import { invoke } from "@tauri-apps/api/tauri";
import { sessions } from "$lib/session";

export type RTCharacter = {
  name: string,
  status: string
}

export function login(username: string, password: string) {
  return invoke("login", { username, password });
}

export function getCharacters(): Promise<string[]> {
  return invoke("get_characters", {});
}

export function getFriends(): Promise<RTCharacter[]> {
  return invoke("get_friends", {});
}

export async function getOnlineFriends(): Promise<string[]> {
  return (await getFriends()).filter((v) => v.status == "online").map((v) => v.name);
}

export async function startSession(character: string) {
  await invoke("start_session", { character: character });
  sessions.update((v) => {
    v.unshift(character);
    return v;
  })
}
