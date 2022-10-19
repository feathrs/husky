import { derived, get, writable } from "svelte/store";
import * as rust from "./rust";
import type { MessageTarget } from "./rust";
import type { Channel, Character } from "./types";

export const sessions = writable<string[]>([]);
export const currentSession = derived(sessions, $sessions => $sessions.at(0) ?? null);

export function changeSession(newSession: string) {
  sessions.update((v) => {
    let swapIndex = v.indexOf(newSession);
    if (swapIndex < 0) {
      // This session doesn't exist!
      throw new Error(`No such session ${newSession}`);
    } else if (swapIndex > 0) {
      // We need to swap it (0 means it's already active)
      let oldPrimary = v[0];
      v[0] = v[swapIndex];
      v[swapIndex] = oldPrimary;
    }
    return v;
  });
}

function thisSession() {
  let session = get(currentSession);
  if (session === null) throw new Error("No active session but attempted to perform session action");
  return session;
}

export async function sendMessage(target: MessageTarget, message: string) {
  await rust.sendMessage(thisSession(), target, message);
}

export async function sendDice(target: MessageTarget, dice: string) {
  await rust.sendDice(thisSession(), target, dice);
}

export async function joinChannel(channel: Channel) {
  await rust.joinChannel(thisSession(), channel);
}
