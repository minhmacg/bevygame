use crate::data_input::{Kdv, MessItems, Messages};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagesOutput {
    time: String,
    sender_name: String,
    content: String,
    photos: String,
    videos: String,
    share: String,
    kdv: Kdv,
}
