use actix::prelude::*;

// add
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddFriend {
    pub user_id: i32,
    pub friend_id: i32,
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

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddUserToRoom {
    pub user_id: i32,
    pub room_id: i32,
}

//leave
#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom {
    pub user_id: i32,
    pub room_id: i32,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveFriend {
    pub user_id: i32,
    pub friend_id: i32,
}

#[derive(Message, Clone)]
#[rtype(result = "String")]
pub struct Message {
    pub msg_content: String,
    pub msg_from: String,
    pub msg_type: MessageType,
    pub msg_to: MessageTo,
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

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub enum MessageType {
    Text,
    Binary,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub enum MessageTo {
    UserMessage(String),
    RoomMessage(String),
}
