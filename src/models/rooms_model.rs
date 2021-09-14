use crate::schema::rooms;
use serde::{Serialize,Deserialize};
#[derive(Queryable,Serialize,Deserialize)]
pub struct Room{
    pub id:i32,
    pub user_id:i32

}
#[derive(Insertable)]
#[table_name="rooms"]
pub struct NewRoom{
    pub user_id:i32,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct PostRoom{
    pub user_id:String,
    pub room_id:String,
}