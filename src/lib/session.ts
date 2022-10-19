import { derived, writable } from "svelte/store";

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
