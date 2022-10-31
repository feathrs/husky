use std::sync::Arc;

use tokio::sync::mpsc::Sender;

use f_chat_rs::{client::{async_trait, EventListener}, data::{Channel, Character}};
use serde::Serialize;
use tauri::{Manager, Runtime};

#[derive(Debug)]
pub struct EventHandler {
    update_emitter: Sender<UpdateEvent>
}
impl EventHandler {
    pub fn new(update_emitter: Sender<UpdateEvent>) -> Self {
        EventHandler { update_emitter }
    }
}

#[derive(Debug)]
pub enum UpdateEvent {
    Bookmarks,
    Channel(Channel),
    Character(Character),
    Friends,

    Message(MessageEvent)
}

#[async_trait]
impl EventListener for EventHandler {
    async fn message(&self, ctx: Arc<Session>, source: &MessageSource, target: &MessageTarget, message: &str) {
        println!("{ctx:?} -- Message from {source:?} to {target:?} -- {message:?}");
        self.update_emitter.send(UpdateEvent::Message(MessageEvent {
            from: match source {
                MessageSource::Character(char) => char.0.clone(),
                MessageSource::System => "_system".to_owned(),
            },
            pm: match target {
                MessageTarget::PrivateMessage(..) => true,
                _ => false
            },
            to: match target {
                MessageTarget::PrivateMessage(char) => char.0.clone(),
                MessageTarget::Channel(chan) => chan.0.clone(),
                MessageTarget::Broadcast => "_all".to_owned()
            },
            content: message.to_owned()
        })).await.expect("Event failed (message)");
    }

    async fn updated_bookmarks(&self) {
        self.update_emitter.send(UpdateEvent::Bookmarks).await.expect("Event failed (updated_bookmarks)");
    }
    async fn updated_channel(&self, channel: Channel) {
        self.update_emitter.send(UpdateEvent::Channel(channel)).await.expect("Event failed (updated_channel)");
    }
    async fn updated_character(&self, user: Character) {
        self.update_emitter.send(UpdateEvent::Character(user)).await.expect("Event failed (updated_character)");
    }
    async fn updated_friends(&self) {
        self.update_emitter.send(UpdateEvent::Friends).await.expect("Event failed (updated_friends)");
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct MessageEvent {
    from: String,
    to: String,
    pm: bool,
    content: String
}

pub async fn handle_event<R: Runtime>(handle: &impl Manager<R>, event: UpdateEvent) {
    match event {
        UpdateEvent::Bookmarks => handle.emit_all("update_bookmark", ()),
        UpdateEvent::Channel(chan) => handle.emit_all("update_channel", chan),
        UpdateEvent::Character(character) => handle.emit_all("update_character", character),
        UpdateEvent::Friends => handle.emit_all("update_friends", ()),
        UpdateEvent::Message(event) => handle.emit_all("message", event),
    }.expect("Failed to emit event");
}
