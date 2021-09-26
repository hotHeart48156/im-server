// pub mod  message_date_format;
use actix::prelude::*;
use chrono::{DateTime, Local};
use serde::{Serialize,Deserialize};

pub mod message_date_format {
    use chrono::{DateTime, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(
        date: &DateTime<chrono::Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // date.timestamp()
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<chrono::Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        chrono::Local.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinOnlineUser {
    pub user_id: i32,
    pub ctx: Recipient<Message>,
}

#[derive(Message, Clone, Copy)]
#[rtype(result = "()")]
pub struct RemoveOnlineUser {
    pub user_id: i32,
}

#[derive(Message, Clone,Serialize,Deserialize)]
#[rtype(result = "String")]
pub struct Message {
    pub msg_content: String,
    pub msg_from: String,
    pub msg_type: MessageType,
    pub msg_to: MessageTo,
    #[serde(with = "message_date_format")]
    pub arrive_time:DateTime<Local> 
}
impl Message {
    pub fn to_string(&self) -> String {
        let msg_type_string: String;
        match self.msg_type {
            MessageType::Text => msg_type_string = "TEXT".to_string(),
            MessageType::Binary => msg_type_string = "BINARY".to_string(),
        }
        let msg_to_string: String;
        match self.msg_to.clone() {
            MessageTo::RoomMessage(id) => msg_to_string = format!("{}/{}", "ROOM", id),
            MessageTo::UserMessage(id) => msg_to_string = format!("{}/{}", "USER", id),
        }
        format!(
            "{}/{}/{}/{}",
            self.msg_content.clone(),
            self.msg_from.clone(),
            msg_type_string,
            msg_to_string
        )
    }
}

#[derive(Message, Clone,Serialize,Deserialize)]
#[rtype(result = "()")]
pub enum MessageType {
    Text,
    Binary,
}

#[derive(Message, Clone,Serialize,Deserialize)]
#[rtype(result = "()")]
pub enum MessageTo {
    UserMessage(String),
    RoomMessage(String),
}
