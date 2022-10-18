// This is where I'm packing all of the functions for talking to Tauri backend
import { invoke } from "@tauri-apps/api/tauri";

export function login(username: string, password: string) {
    return invoke("login", { username, password });
}

export function getCharacters(): Promise<string[]> {
    return invoke("get_characters", {});
}

export function startSession(character: string) {
    return invoke("start_session", { character: character });
}
