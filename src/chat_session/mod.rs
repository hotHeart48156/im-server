use crate::chat_server::ChatServer;
use crate::message::{JoinOnlineUser, SendMessageToFriend, SendMessageToRoom};
use actix::prelude::*;
use actix::Handler;
use actix::{Actor, StreamHandler};
use actix::{ActorContext, SystemService};
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
#[derive(Default)]
pub struct UserSession {
    pub user_id:String
}
impl UserSession {
    pub fn send_message_to_room(&self, user_id: i32, content: &str, roomid: i32) {
        let id: i32 = rand::random();
        let msg = SendMessageToRoom {
            id: id.to_string(),
            msg: content.to_string(),
            user_id: user_id,
            room_id: roomid,
        };
        self.issue_system_async(msg);
    }
    pub fn send_message_to_friend(&self,  userid: i32,content: &str, friendid: i32) {
        let id: i32 = rand::random();
        let msg = SendMessageToFriend {
            id: id.to_string(),
            msg: content.to_string(),
            friend_id: friendid,
            user_id: userid,
        };
        self.issue_system_async(msg);
    }
    pub fn join_online_user(&self, userid: i32, ctx: Recipient<SendMessageToFriend>) {
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

impl Handler<SendMessageToFriend> for UserSession {
    type Result = ();
    fn handle(&mut self, msg: SendMessageToFriend, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.msg);
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
                            if let Some(msg) = command.next() {
                                recv_msg = msg;
                            }
                            if let Some(friend_id) = command.next() {
                                recv_friend_id = friend_id.parse::<i32>().unwrap();
                            }
                            ctx.text(recv_msg);
                            self.send_message_to_friend(self.user_id.parse::<i32>().unwrap(),recv_msg , recv_friend_id)
                        }
                        Some("send_message_to_room") => {
                            let mut recv_msg = "";
                            let mut recv_room_id: i32 = 0;
                
                            if let Some(msg) = command.next() {
                                recv_msg = msg;
                            }
                            if let Some(room_id) = command.next() {
                                recv_room_id = room_id.parse::<i32>().unwrap();
                            }

                            self.send_message_to_room(self.user_id.parse::<i32>().unwrap(), recv_msg, recv_room_id);
                            ctx.text(recv_msg)
                        }

                        Some("regist_online_user") => {
                            let mut userid = "";
                            if let Some(user_id) = command.next() {
                                userid = user_id;
                            }
                            println!("id--{}", userid);
                            self.join_online_user(
                                userid.parse::<i32>().unwrap(),
                                ctx.address().recipient(),
                            )
                        }
                        Some(msg) => {
                            println!("not found message{}", msg)
                        }
                        None => {}
                    }
                }
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
                
            }
            _ => {}
        };
    }
}
