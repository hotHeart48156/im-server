use crate::message::{JoinOnlineUser, SendMessageToFriend, SendMessageToRoom};
use actix::prelude::*;
use actix::{Actor, Context};
use actix_broker::BrokerSubscribe;
use std::collections::HashMap;
type RoomClient = Recipient<SendMessageToRoom>;
type Room = HashMap<usize, RoomClient>;
type OnlineClient = Recipient<SendMessageToFriend>;

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
        let msg = SendMessageToRoom {
            msg: msg.to_string(),
            user_id: userid,
            id: msgid,
            room_id: roomid,
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
        let msg = SendMessageToFriend {
            msg: msg.to_string(),
            id: msgid,
            friend_id: friendid,
            user_id: userid,
        };
        let friend=self.online_users.get(&userid.to_string());
        if let Some(online_user) = friend {
            online_user.do_send(msg).unwrap();
        }
        // if let None = friend {
        //     todo!("search user on database if find write message to database ")
        // }
    }
    pub fn read_message(&self,_userid:i32){
        // todo!("for offline user read message ");
        // todo!("get all of user message id to set");
        // todo!("set ");
    }
    pub fn join_online_user(&mut self, userid: i32, ctx: Recipient<SendMessageToFriend>) {
        let user=self.online_users.get(&userid.to_string()) ;
        if None==user{
            self.online_users.insert(userid.to_string(), ctx);

        }else {
            let id:i32=rand::random();
            let msg=SendMessageToFriend{
                id:id.to_string(),
                msg:"alread register to online user".to_string(),
                friend_id:userid,
                user_id:userid
            };
            if let Some(user) =user  {
                user.do_send(msg).unwrap();
            }
        }
       
    }

    pub fn list_friend() {}
    pub fn list_rooms() {}
}

impl Actor for ChatServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("chat server started");
        self.subscribe_system_async::<SendMessageToRoom>(ctx);
        self.subscribe_system_async::<SendMessageToFriend>(ctx);
    }
}

impl Handler<SendMessageToFriend> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: SendMessageToFriend, _ctx: &mut Self::Context) -> Self::Result {
        self.send_message_to_friend(msg.user_id, msg.friend_id, msg.msg.as_str());
    }
}

impl Handler<SendMessageToRoom> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: SendMessageToRoom, _ctx: &mut Self::Context) -> Self::Result {
        println!("handle SendMessage To Room ");
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
