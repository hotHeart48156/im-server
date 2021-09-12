use crate::schema::friends;
#[derive(Queryable)]
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
