use crate::schema::friends;
use serde::{Serialize,Deserialize};
#[derive(Queryable,Serialize,Deserialize)]
pub struct Friend {
    pub id: i32,
    pub user_id: i32,
    pub friend_id: i32,
}
#[derive(Insertable)]
#[table_name = "friends"]
pub struct NewFriend {
    pub user_id: i32,
    pub friend_id: i32,
}
#[derive(Debug,Serialize,Deserialize,Clone, Copy)]
pub struct PostFriend {
    pub user_id: i32,
    pub friend_id: i32,
}
