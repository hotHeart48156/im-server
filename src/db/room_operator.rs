use crate::models::rooms_model::{NewRoom, Room};
use crate::schema::rooms;
use crate::schema::rooms::dsl::*;
use crate::server::DbPoolType;
use diesel::prelude::*;
pub struct RoomsOperator<'a> {
    pub conn: &'a DbPoolType,
}

impl<'a> RoomsOperator<'a> {
    pub fn list_room(&self, userid: i32) -> Option<Vec<Room>> {
        let result = rooms
            .filter(user_id.eq(&userid))
            .get_results::<Room>(&self.conn.get().unwrap());

        match result {
            Ok(ok) => Some(ok),
            Err(_) => None,
        }
    }

    pub fn create_room(&self, userid: i32) -> Option<Room> {
        let new_room = NewRoom { user_id: userid };
        let result: Result<Room, diesel::result::Error> = diesel::insert_into(rooms::table)
            .values(&new_room)
            .get_result::<Room>(&self.conn.get().unwrap());
        match result {
            Ok(ok) => Some(ok),
            Err(_) => None,
        }
    }
    pub fn delete_room(&self, userid: i32, roomid: i32) -> Option<usize> {
        let result = diesel::delete(rooms.filter(user_id.eq(userid).and(id.eq(roomid))))
            .execute(&self.conn.get().unwrap());

        match result {
            Ok(ok) => Some(ok),
            Err(_) => None,
        }
    }
}
