#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use serde::Serialize;
use tauri::{async_runtime::RwLock as AsyncRwLock, State};
use f_chat_rs::{client::{Client, EventListener, Session, MessageSource, MessageTarget}, data::{Character, Channel, Status}};

// https://github.com/tauri-apps/tauri/issues/2533
type AsyncVoid = Result<(),()>;

// client::ClientError is not yet Serialize, so cannot be used.
#[tauri::command]
async fn login(client: State<'_, MaybeClient>, username: String, password: String) -> AsyncVoid {
    let mut new_client = Client::new(username, password, "Husky".to_owned(), "0.1".to_owned(), EventHandler);
    new_client.init().await.expect("Failed to init client!"); // This might cause this function to blow up on an incorrect user/pass. Reconsider.
    // Also start the client recieving events in the background.
    let new_client = Arc::new(dbg!(new_client));
    *client.client.write().await = Some(new_client.clone());
    tokio::spawn(async move { new_client.start().await });
    
    Ok(())
}

#[tauri::command]
async fn get_characters(client: State<'_, MaybeClient>) -> Result<Vec<Character>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic!");
    Ok(client.own_characters.left_values().cloned().collect())
}

#[derive(Serialize)]
struct RTCharacter {
    name: Character,
    status: Status
}

#[tauri::command]
async fn get_friends(client: State<'_, MaybeClient>) -> Result<Vec<RTCharacter>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic.");
    let friends_guard = client.friends.read();
    let friends = friends_guard.iter().map(|v| RTCharacter {
        name: v.clone(),
        status: client.character_data.get(v)
            .map(|c|c.status)
            .unwrap_or(Status::Offline) // Default is not Offline and that's my fault.
    }).collect();

    Ok(friends)
}

#[tauri::command]
async fn start_session(client: State<'_, MaybeClient>, character: Character) -> AsyncVoid {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic...");
    client.connect(character).await.expect("Failed to connect character");
    Ok(())
}

struct MaybeClient {
    client: AsyncRwLock<Option<Arc<Client<EventHandler>>>> // This is ugly.
}

#[derive(Debug)]
struct EventHandler;
impl EventListener for EventHandler {
    fn message(&self, ctx: Arc<Session>, source: &MessageSource, target: &MessageTarget, message: &str) {
        println!("{ctx:?} -- Message from {source:?} to {target:?} -- {message:?}");
    }
}

#[tokio::main]
async fn main() {
    let client = MaybeClient {client: AsyncRwLock::new(None)};

    // This isn't necessary but I want to control the async runtime
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![login, get_characters, get_friends, start_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
