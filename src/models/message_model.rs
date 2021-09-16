use crate::schema::message;
#[derive(Debug,Queryable)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub from_id: i32,
    pub message_type: String,
    pub message_content: Option<String>,
}

#[derive(Debug,Insertable)]
#[table_name="message"]
pub struct NewMessage {
    pub user_id: i32,
    pub from_id: i32,
    pub message_type: String,
    pub message_content: Option<String>,
}
