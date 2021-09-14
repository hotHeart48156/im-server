use crate::models::friend_model::Friend;
use crate::server::DbPoolType;
use crate::{models::friend_model::NewFriend, schema::friends, schema::friends::dsl::*};
use diesel::prelude::*;
pub struct FriendOperator<'a> {
    pub conn: &'a DbPoolType,
}
impl<'a> FriendOperator<'a> {
    pub fn add_friends(&self, new_friend:NewFriend) -> Result<Friend, diesel::result::Error> {
        // let new_friend = NewFriend {
        //     friend_id: friendid,
        //     user_id: userid,
        // };
        diesel::insert_into(friends::table)
            .values(&new_friend)
            .get_result::<Friend>(&self.conn.get().unwrap())
    }

    pub fn delete_friend(&self, userid: i32, friendid: i32) -> Option<usize>  {
       let result= diesel::delete(friends.filter(user_id.eq(&userid).and(friend_id.eq(&friendid))))
            .execute(&self.conn.get().unwrap());
            match result {
                Ok(ok) => {Some(ok)},
                Err(_) => {None},
            }

            
    }
    pub fn list_friends(&self, userid: i32)->Option<Vec<Friend>> {
        let result_friends= friends
            .filter(user_id.eq(userid))
            .get_results::<Friend>(&self.conn.get().unwrap());
            match result_friends {
                Ok(recv_friends) => {Some(recv_friends)},
                Err(_) => {None},
            }
    }
}
