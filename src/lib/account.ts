// Svelte store mapped to persistent storage.
import { browser } from "$app/environment";
import { writable } from "svelte/store"

export const username = writable(browser ? window.localStorage.getItem("username") : null);
export const password = writable(browser ? window.localStorage.getItem("password") : null);
export const autoLogin = writable(browser ? window.localStorage.getItem("auto_login") === "true" : false);

// I solemnly swear that the value will never be set null by my code.
if (browser) {
  username.subscribe((v) => {if (v) {localStorage.setItem("username", v)}});
  password.subscribe((v) => {if (v) {localStorage.setItem("password", v)}});
  autoLogin.subscribe((v) => localStorage.setItem("auto_login", v.toString()))
}
