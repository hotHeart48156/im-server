use actix::prelude::*;

// add
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddFriend{
    pub user_id:i32,
    pub friend_id:i32
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinOnlineUser{
    pub user_id:i32,
    pub ctx:Recipient<SendMessageToFriend>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddUserToRoom{
    pub user_id:i32,
    pub room_id:i32
}

//leave
#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom{
    pub user_id:i32,
    pub room_id:i32

}
#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveFriend{
    pub user_id:i32,
    pub friend_id:i32

}

//send
#[derive(Message,Clone)]
#[rtype(result = "()")]
pub struct SendMessageToRoom{
    pub id:String,
    pub msg:String,
    pub user_id:i32,
    pub room_id:i32
}

#[derive(Message,Clone)]
#[rtype(result = "()")]
pub struct SendMessageToFriend{
    pub id:String,
    pub msg:String,
    pub friend_id:i32,
    pub user_id:i32,
    
}





