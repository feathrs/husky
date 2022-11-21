// On the frontend, we have to hold copies of the data that's on the backend.
// Not all of it, just enough to display. But it needs to be held.
// At a future date, the "just enough to display" can be shrunk down massively to save memory.
// But for now, it's a bit more than we'll ever display.

import { writable } from "svelte/store";
import { listen } from '@tauri-apps/api/event';
import type { Event } from '@tauri-apps/api/event';
import { getFriends, getBookmarks, getChannel, getCharacter, getAllCharacters } from "$lib/rust";
import type { Channel, Character, ChannelData, CharacterData } from "./types";
import { browser } from "$app/environment";

export const bookmarks = writable<Character[]>([]);
export const friends = writable<Character[]>([]);
export const channels = writable<Record<Channel, ChannelData>>({});
export const characters = writable<Record<Character, CharacterData>>({});

if (browser) {
  listen('update_bookmark', syncBookmarks)
  listen('update_friends', syncFriends)
  listen('update_channel', async (e: Event<Channel>) => {
    let data = await getChannel(e.payload);
    channels.update((v) => {
      v[e.payload] = data;
      return v;
    });
  })
  listen('update_character', async (e: Event<Character>) => {
    let data = await getCharacter(e.payload);
    characters.update((v) => {
      v[e.payload] = data;
      return v;
    })
  })
  syncBookmarks();
  syncFriends();
  syncCharacters();
}

export async function syncBookmarks() {
  bookmarks.set(await getBookmarks());
}

export async function syncFriends() {
  friends.set(await getFriends());
}

export async function syncCharacters() {
  let chars = await getAllCharacters();
  console.log(chars);
  characters.set(chars);
}

// No syncChannels?
// Hopefully not. I'll see if I need it, but I'd prefer not to.
// Mostly because of how I distribute channel membership.
