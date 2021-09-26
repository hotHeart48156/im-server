use crate::schema::message;
#[derive(Debug,Queryable)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub destination_id: i32,
    pub message_type: String,
    pub message_content:String,
    pub destination_type: String,
    pub arrive_time:chrono::NaiveDateTime 

    
}

#[derive(Debug,Insertable)]
#[table_name="message"]
pub struct NewMessage<'a> {
    pub user_id: i32,
    pub destination_id: i32,
    pub message_type: &'a str,
    pub message_content: &'a str,
    pub destination_type: &'a str,
    pub arrive_time:chrono::NaiveDateTime 

}
