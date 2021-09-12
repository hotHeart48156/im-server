use crate::schema::room_members;
#[derive(Queryable)]
pub struct RoomMember{
    pub id:i32,
    pub room_id:i32,
    pub member_id:i32,
}
#[derive(Insertable)]
#[table_name="room_members"]
pub struct NewRoomMember{
    pub room_id:i32,
    pub member_id:i32,
}