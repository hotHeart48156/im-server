use crate::models::rooms_model::{NewRoom, Room};
use crate::schema::rooms;
use crate::schema::rooms::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
pub struct RoomsOperator<'a> {
    pub conn: &'a PgConnection,
}

impl<'a> RoomsOperator<'a> {
    pub fn get_rooms_by_user_id(&self, userid: i32) -> Vec<Room> {
        rooms
            .filter(user_id.eq(&userid))
            .get_results(self.conn)
            .expect("cant found room")
    }

    pub fn crate_room(&self, userid: i32)->Room {
        let new_room = NewRoom { user_id: userid };
        diesel::insert_into(rooms::table)
            .values(&new_room)
            .get_result::<Room>(self.conn)
            .expect("")
    }
}
