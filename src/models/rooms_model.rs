use crate::schema::rooms;
#[derive(Queryable)]
pub struct Room{
    pub id:i32,
    pub user_id:i32,

}
#[derive(Insertable)]
#[table_name="rooms"]
pub struct NewRoom{
    pub user_id:i32,
}