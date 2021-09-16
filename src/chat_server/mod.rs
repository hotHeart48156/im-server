use crate::message::{JoinOnlineUser, SendMessageToFriend, SendMessageToRoom};
use actix::prelude::*;
use actix::{Actor, Context};
use actix_broker::BrokerSubscribe;
use std::collections::HashMap;
use crate::message;
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
        let msgid = format!("{:?}", std::time::Instant::now());
        let msg = message::Message {
            msg_content: msg.to_string(),
            msg_from: userid.to_string(),
            id: msgid,
            msg_to: roomid.to_string(),
            msg_type:"room_message".to_string()
        };
        if let Some(room) = self.rooms.get(&roomid.to_string()) {
            let msg = msg.clone();
            for (_roomclientid, roomclient) in room {
                roomclient.do_send(msg.to_owned()).unwrap();
            }
        }
    }

    pub fn send_message_to_friend(&self, userid: i32, friendid: i32, msg: &str) {
        let msgid = format!("{}-{:?}", friendid,std::time::Instant::now());
        let msg = message::Message {
            msg_content: msg.to_string(),
            id: msgid,
            msg_to: friendid.to_string(),
            msg_from: userid.to_string(),
            msg_type:"user_message".to_string()
        };

        let friend=self.online_users.get(&friendid.to_string());
        if let Some(online_user) = friend {
            online_user.do_send(msg).unwrap();
        }
   
    }
    pub fn read_message(&self,_userid:i32){
      
    }
    pub fn join_online_user(&mut self, userid: i32, ctx: Recipient<message::Message>) {
        let user=self.online_users.get(&userid.to_string()) ;
        if None==user{
            self.online_users.insert(userid.to_string(), ctx);

        }else {
            let id:i32=rand::random();
            let msg=message::Message{
                id:id.to_string(),
                msg_content:"alread register to online user".to_string(),
                msg_from:userid.to_string(),
                msg_to:userid.to_string(),
                msg_type:"user_message".to_string()
            };
            if let Some(user) =user  {
                user.do_send(msg).unwrap();
            }
        }
       
    }

}

impl Actor for ChatServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("chat server started");
        self.subscribe_system_async::<SendMessageToRoom>(ctx);
        self.subscribe_system_async::<message::Message>(ctx);
    }
}

impl Handler<message::Message> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: message::Message, _ctx: &mut Self::Context) -> Self::Result {
        self.send_message_to_friend(msg.msg_from.parse::<i32>().unwrap(), msg.msg_to.parse::<i32>().unwrap(), msg.msg_content.as_str());
    }
}

impl Handler<SendMessageToRoom> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: SendMessageToRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.send_message_to_room(msg.user_id, msg.room_id, msg.msg.as_str())
    }
}



impl Handler<JoinOnlineUser> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: JoinOnlineUser, _ctx: &mut Self::Context) -> Self::Result {
        self.join_online_user(msg.user_id, msg.ctx);
    }
}
impl SystemService for ChatServer {}
impl Supervised for ChatServer {}
