use crate::db::message_operation::read_unrecived_message_no_actix;
use crate::db::message_operation::MessageOperator;
use crate::db::user_operator::get_user_by_id_nodb;
use crate::message::JoinOnlineUser;
use crate::message::RemoveOnlineUser;
use crate::models::message_model::{Message, NewMessage};
use crate::{establish_connection, message};
use actix::prelude::*;
use actix::{Actor, Context};
use actix_broker::BrokerSubscribe;
use std::collections::HashMap;
type RoomClient = Recipient<message::Message>;
type Room = HashMap<usize, RoomClient>;
type OnlineClient = Recipient<message::Message>;

#[derive(Default)]
pub struct ChatServer {
    pub rooms: HashMap<String, Room>,
    pub online_users: HashMap<String, OnlineClient>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer {
            rooms: HashMap::new(),
            online_users: HashMap::new(),
        }
    }

    pub fn join_room(&mut self, userid: i32, roomid: i32, client: RoomClient) -> i32 {
        let id = format!("{}{}", roomid, userid);
        if let Some(room) = self.rooms.get_mut(&roomid.to_string()) {
            if room.contains_key(&(userid as usize)) {
                room.insert(id.parse::<usize>().unwrap(), client);
            }
        }
        id.parse::<i32>().unwrap()
    }
    pub fn notify(&self) {}

    pub fn send_message_to_room(&self, userid: i32, roomid: i32, msg: &str) {
        let msg = message::Message {
            msg_content: msg.to_string(),
            msg_from: userid.to_string(),
            msg_to: message::MessageTo::RoomMessage(roomid.to_string()),
            msg_type: message::MessageType::Text,
        };
        if let Some(room) = self.rooms.get(&roomid.to_string()) {
            let msg = msg.clone();
            for (_roomclientid, roomclient) in room {
                roomclient.do_send(msg.to_owned()).unwrap();
            }
        }
    }
    pub fn write_new_message_to_db(&self, new_message: NewMessage) {
        let conn = establish_connection();
        let msg_operator = MessageOperator { conn: &conn };
        msg_operator.new_message(&new_message).unwrap();
    }
    pub fn user_is_exist(&self, userid: i32) -> bool {
        let conn = establish_connection();
        match get_user_by_id_nodb(userid, &conn) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn send_message_to_friend(
        &self,
        userid: i32,
        friendid: i32,
        content: &str,
        msg_type: message::MessageType,
    ) {
        let msg_to_online_user = message::Message {
            msg_content: content.to_string(),
            msg_to: message::MessageTo::UserMessage(friendid.to_string()),
            msg_from: userid.to_string(),
            msg_type: msg_type.clone(),
        };
        let mut  message_type:String=String::from("");
        match msg_type {
            message::MessageType::Text => message_type = "TEXT".to_string(),
            message::MessageType::Binary => message_type = "BINARY".to_string(),
        }
        println!("{}",message_type);
        let new_message_database = NewMessage {
            user_id: userid,
            destination_id: friendid,
            message_type: message_type.as_str(),
            message_content:content,
            destination_type:"USER"
        };

        let friend = self.online_users.get(&friendid.to_string());
        if let Some(online_user) = friend {
            online_user.do_send(msg_to_online_user).unwrap();
            self.write_new_message_to_db(new_message_database)
        } else {
            match self.user_is_exist(userid) {
                true => self.write_new_message_to_db(new_message_database),
                false => {}
            }
        }
    }
    pub fn read_message(&self, userid: i32, message_index: i32) -> Option<Vec<Message>> {
        let conn = establish_connection();
        match read_unrecived_message_no_actix(userid, message_index, &conn) {
            Some(messages) => Some(messages),
            None => None,
        }
    }
    pub fn remove_user(&mut self,userid:i32){
        self.online_users.remove(&userid.to_string());
    }
    pub fn join_online_user(&mut self, userid: i32, ctx: Recipient<message::Message>) {
        let user = self.online_users.get(&userid.to_string());
        if None == user {
            self.online_users.insert(userid.to_string(), ctx);
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<message::Message>(ctx);
        self.subscribe_system_async::<message::RemoveOnlineUser>(ctx);
    }
}

impl Handler<message::Message> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: message::Message, _ctx: &mut Self::Context) -> Self::Result {
        if let message::MessageTo::UserMessage(id) = msg.msg_to.clone() {
            self.send_message_to_friend(
                msg.msg_from.parse::<i32>().unwrap(),
                id.parse::<i32>().unwrap(),
                msg.msg_content.as_str(),
                msg.msg_type.clone(),
            )
        }

        if let message::MessageTo::RoomMessage(id) = msg.msg_to {
            if let message::MessageType::Text = msg.msg_type {
                self.send_message_to_room(
                    msg.msg_from.parse::<i32>().unwrap(),
                    id.parse::<i32>().unwrap(),
                    msg.msg_content.as_str(),
                );
            }
        }
    }
}

impl Handler<JoinOnlineUser> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: JoinOnlineUser, _ctx: &mut Self::Context) -> Self::Result {
        self.join_online_user(msg.user_id, msg.ctx);
    }
}

impl Handler<RemoveOnlineUser> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: RemoveOnlineUser, _ctx: &mut Self::Context) -> Self::Result {
        self.remove_user(msg.user_id);
    }
}
impl SystemService for ChatServer {}
impl Supervised for ChatServer {}
