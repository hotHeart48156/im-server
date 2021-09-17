use crate::chat_server::ChatServer;
use crate::message::{self, JoinOnlineUser, MessageType, RemoveOnlineUser};

use actix::prelude::*;
use actix::Handler;
use actix::{Actor, StreamHandler};
use actix::{ActorContext, SystemService};
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
pub struct UserSession {
    pub user_id: String,
}
impl UserSession {
    pub fn send_message_to_room(
        &self,
        user_id: i32,
        content: &str,
        roomid: i32,
        msg_type: MessageType,
    ) {
        let msg = message::Message {
            msg_content: content.to_string(),
            msg_from: user_id.to_string(),
            msg_to: message::MessageTo::RoomMessage(roomid.to_string()),
            msg_type: msg_type,
        };

        ChatServer::from_registry().do_send(msg);
    }
    pub fn send_message_to_friend(
        &self,
        userid: i32,
        content: &str,
        friendid: i32,
        msg_type: MessageType,
    ) {
        let msg = message::Message {
            msg_content: content.to_string(),
            msg_to: message::MessageTo::UserMessage(friendid.to_string()),
            msg_from: userid.to_string(),
            msg_type: msg_type,
        };
        ChatServer::from_registry().do_send(msg)
    }
    pub fn join_online_user(&self, userid: i32, ctx: Recipient<message::Message>) {
        let msg = JoinOnlineUser {
            user_id: userid,
            ctx: ctx,
        };
        ChatServer::from_registry().do_send(msg)
    }
}

impl Actor for UserSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        ChatServer::from_registry();
    }
}

impl Handler<message::Message> for UserSession {
    type Result = MessageResult<message::Message>;
    // type Result = ();

    fn handle(&mut self, msg: message::Message, _ctx: &mut Self::Context) -> Self::Result {
        let tt=msg.clone();
        _ctx.text(msg.to_string());//发送消息关键
        MessageResult(tt.to_string())
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(_) => {
                return;
            }
        };

        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();
                if msg.starts_with("/") {
                    let mut command = msg.split("/");
                    command.next();
                    match command.next() {
                        Some("send_message_to_friend") => {
                            let mut recv_msg = "";
                            let mut recv_friend_id: i32 = 0;
                            let mut recv_msg_type: MessageType = MessageType::Text;
                            if let Some(msg) = command.next() {
                                recv_msg = msg;
                            }
                            if let Some(friend_id) = command.next() {
                                recv_friend_id = friend_id.parse::<i32>().unwrap();
                            }
                            if let Some(msg_type) = command.next() {
                                match msg_type {
                                    "TEXT" => recv_msg_type = MessageType::Text,
                                    "BINARY" => recv_msg_type = MessageType::Binary,
                                    _ => {
                                        ctx.text("message type not correct");
                                        ctx.stop()
                                    }
                                }
                            }
                            ctx.text(recv_msg);
                            self.join_online_user(
                                self.user_id.parse::<i32>().unwrap().clone(),
                                ctx.address().recipient(),
                            );
                            self.send_message_to_friend(
                                self.user_id.parse::<i32>().unwrap(),
                                recv_msg,
                                recv_friend_id,
                                recv_msg_type,
                            )
                        }
                        Some("send_message_to_room") => {
                            let mut recv_msg = "";
                            let mut recv_room_id: i32 = 0;
                            let mut recv_msg_type: MessageType = MessageType::Text;

                            if let Some(msg) = command.next() {
                                recv_msg = msg;
                            }
                            if let Some(room_id) = command.next() {
                                recv_room_id = room_id.parse::<i32>().unwrap();
                            }
                            if let Some(msg_type) = command.next() {
                                match msg_type {
                                    "TEXT" => recv_msg_type = MessageType::Text,
                                    "BINARY" => recv_msg_type = MessageType::Binary,
                                    _ => {
                                        ctx.text("message type not correct");
                                        ctx.stop()
                                    }
                                }
                            }
                            self.join_online_user(
                                self.user_id.parse::<i32>().unwrap().clone(),
                                ctx.address().recipient(),
                            );
                            self.send_message_to_room(
                                self.user_id.parse::<i32>().unwrap(),
                                recv_msg,
                                recv_room_id,
                                recv_msg_type,
                            );
                            ctx.text(recv_msg)
                        }

                        Some("register_online_user") => {
                            let id = self.user_id.clone();
                            ctx.text(id);
                            self.join_online_user(
                                self.user_id.parse::<i32>().unwrap().clone(),
                                ctx.address().recipient(),
                            );
                        }
                        Some(msg) => {
                            println!("not found message{}", msg)
                        }
                        None => {}
                    }
                }
            }
            ws::Message::Close(reason) => {
                println!("user leave {}",self.user_id);
                self.issue_system_async(RemoveOnlineUser {
                    user_id: self.user_id.parse::<i32>().unwrap(),
                });
                ctx.close(reason);
                ctx.stop();
            }

            _ => {}
        };
    }
}
