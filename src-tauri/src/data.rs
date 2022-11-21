use f_chat_rs::data::{Gender, Status};
use serde::Serialize;

// For instances where there's data which needs to go to the frontend,
// but for some reason I can't use one of the existing types (e.g. it's redundant or the wrong format)
use std::default::Default;
#[derive(serde::Serialize, Debug, Default)]
pub struct CharacterDataInner {
    pub gender: Gender,
    pub status: Status,
    pub status_message: String,
}
