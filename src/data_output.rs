use crate::data_input::{Kdv,Messages,MessItems};
use serde::{Serialize,Deserialize};
use serde_json::{Value, Number};

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


