use crate::models::room_member_model::RoomMember;
use crate::schema::room_members::dsl::*;
use crate::{models::room_member_model::NewRoomMember, schema::room_members};
use diesel::prelude::*;
use diesel::PgConnection;
pub struct RoomMemberOperator<'a> {
    pub conn: &'a PgConnection,
}

impl<'a> RoomMemberOperator<'a> {
    pub fn add_room_member(&self, roomid: i32, userid: i32) -> RoomMember {
        let new_room_member = NewRoomMember {
            room_id: roomid,
            member_id: userid,
        };
        diesel::insert_into(room_members::table)
            .values(&new_room_member)
            .get_result(self.conn)
            .expect("cant add user to room")
    }

    pub fn remove_room_member(&self, roomid: i32, userid: i32) -> usize {
        diesel::delete(room_members.filter(room_id.eq(&roomid).and(member_id.eq(&userid))))
            .execute(self.conn)
            .expect("c")
    }
}
