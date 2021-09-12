use crate::models::friend_model::Friend;
// use crate::schema::friends;
use crate::{models::friend_model::NewFriend, schema::friends, schema::friends::dsl::*};
use diesel::prelude::*;
use diesel::PgConnection;
pub struct FriendOperator<'a> {
    pub conn: &'a PgConnection,
}
impl<'a> FriendOperator<'a> {
    pub fn add_friends(&self, userid: i32, friendid: i32) -> Friend {
        let new_friend = NewFriend {
            friend_id: friendid,
            user_id: userid,
        };
        diesel::insert_into(friends::table)
            .values(&new_friend)
            .get_result::<Friend>(self.conn)
            .expect("cant insert friend")
    }

    pub fn delete_friends(&self, userid: i32, friendid: i32) ->usize{
        diesel::delete(friends.filter(user_id.eq(&userid).and(friend_id.eq(&friendid))))
            .execute(self.conn)
            .expect("cant delete friend")
    }
}
