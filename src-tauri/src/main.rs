#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::Arc, collections::BTreeMap};

use tokio::sync::{mpsc::Sender, RwLock as AsyncRwLock};
use tauri::{State};
use f_chat_rs::{client::{Client, CharacterData, ChannelData, MessageTarget as SessionMessageTarget}, data::{Character, Channel, Status}};

mod event;

// https://github.com/tauri-apps/tauri/issues/2533
type AsyncVoid = Result<(),()>;

// This is going to be used in almost every command.
struct MaybeClient {
    client: AsyncRwLock<Option<Arc<Client<event::EventHandler>>>> // This is ugly.
}
type ClientState<'a> = State<'a, MaybeClient>;

// client::ClientError is not yet Serialize, so cannot be used.
#[tauri::command]
async fn login(client: ClientState<'_>, emitter: State<'_, Sender<event::UpdateEvent>>, username: String, password: String) -> AsyncVoid {
    let mut new_client = Client::new(username, password, "Husky".to_owned(), "0.1".to_owned(), event::EventHandler::new(emitter.inner().clone()));
    new_client.init().await.expect("Failed to init client!"); // This might cause this function to blow up on an incorrect user/pass. Reconsider.
    // Also start the client recieving events in the background.
    let new_client = Arc::new(dbg!(new_client));
    *client.client.write().await = Some(new_client.clone());
    tokio::spawn(async move { new_client.start().await });
    
    Ok(())
}

#[tauri::command]
async fn start_session(client: ClientState<'_>, character: Character) -> AsyncVoid {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic... (start_session)");
    client.connect(character).await.expect("Failed to connect character");
    Ok(())
}

#[tauri::command]
async fn get_own_characters(client: ClientState<'_>) -> Result<Vec<Character>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic! (get_own_characters)");
    Ok(client.own_characters.left_values().cloned().collect())
}

#[tauri::command]
async fn get_friends(client: ClientState<'_>) -> Result<Vec<Character>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (get_friends)");

    let friends_guard = client.friends.read();
    Ok(friends_guard.clone())
}

#[tauri::command]
async fn get_bookmarks(client: ClientState<'_>) -> Result<Vec<Character>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (get_bookmarks)");

    Ok(client.bookmarks.clone())
}

#[tauri::command]
async fn get_all_characters(client: ClientState<'_>) -> Result<BTreeMap<Character, CharacterData>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (get_all_characters)");

    // I need to clone it anyway, and it's currently a DashMap, so turn it into a BTreeMap
    Ok(client.character_data.iter().map(|v| (v.key().clone(), v.value().clone())).collect()) 
}

#[tauri::command]
async fn get_character(client: ClientState<'_>, character: Character) -> Result<Option<CharacterData>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (get_character)");

    Ok(client.character_data.get(&character).map(|v|v.clone()))
}

#[tauri::command]
async fn get_channel(client: ClientState<'_>, channel: Channel) -> Result<Option<ChannelData>, ()> {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (get_channel)");

    Ok(client.channel_data.get(&channel).map(|v|v.clone()))
}

#[tauri::command]
async fn get_sessions(client: ClientState<'_>) -> Result<Vec<Character>, ()> {
    let client_guard = client.client.read().await;
    Ok(client_guard.as_ref().map_or(
        Vec::new(), 
        |client| client.get_sessions().drain(..).map(|v| v.character.clone()).collect()
    ))
}

#[derive(serde::Deserialize)]
enum MessageTarget {
    Channel {channel: Channel},
    Character {character: Character}
}

#[tauri::command]
async fn session_send_message(client: ClientState<'_>, session: Character, target: MessageTarget, message: String) -> AsyncVoid {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (session_send_message)");

    let session = client.get_session(&session).expect("Bad session provided (session_send_message)");
    session.send_message(match target {
        MessageTarget::Channel { channel } => SessionMessageTarget::Channel(channel),
        MessageTarget::Character { character } => SessionMessageTarget::PrivateMessage(character)
    }, message).await.expect("Client error (session_send_message)");
    Ok(())
}

#[tauri::command]
async fn session_send_dice(client: ClientState<'_>, session: Character, target: MessageTarget, dice: String) -> AsyncVoid {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (session_send_dice)");

    let session = client.get_session(&session).expect("Bad session provided (session_send_dice)");
    session.send_dice(match target {
        MessageTarget::Channel { channel } => SessionMessageTarget::Channel(channel),
        MessageTarget::Character { character } => SessionMessageTarget::PrivateMessage(character)
    }, dice).await.expect("Client error (session_send_dice)");
    Ok(())
}

#[tauri::command]
async fn session_join_channel(client: ClientState<'_>, session: Character, channel: Channel) -> AsyncVoid {
    let client_guard = client.client.read().await;
    let client = client_guard.as_ref().expect("Too optimistic (join_channel)");

    let session = client.get_session(&session).expect("Bad session provided (join_channel)");
    session.join_channel(channel).await.expect("Client error (join_channel)");
    Ok(())
}

#[tokio::main]
async fn main() {
    let client = MaybeClient {client: AsyncRwLock::new(None)};
    let (send, mut receive) = tokio::sync::mpsc::channel::<event::UpdateEvent>(8);

    // This isn't necessary but I want to control the async runtime
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(client)
        .manage(send)
        .setup(|app| {
            // Oh, and here's where I read off the receiver.
            let handle = app.handle();
            tokio::spawn(async move {
                while let Some(event) = receive.recv().await {
                    event::handle_event(&handle, event).await;
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            login, 
            start_session,
            get_own_characters, 
            get_friends, 
            get_bookmarks,
            get_channel,
            get_character,
            get_all_characters,
            get_sessions,
            session_send_message,
            session_send_dice,
            session_join_channel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
