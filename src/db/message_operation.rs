use crate::schema::message;
use crate::schema::message::dsl::*;
use crate::{models::message_model, models::message_model::NewMessage, server::DbPoolType};
use diesel::prelude::*;
pub struct MessageOperator<'a> {
    pub conn: &'a DbPoolType,
}

impl<'a> MessageOperator<'a> {
    pub fn new_message(
        &self,
        new_message: &NewMessage,
    ) -> Result<message_model::Message, diesel::result::Error> {
        diesel::insert_into(message::table)
            .values(new_message)
            .get_result::<message_model::Message>(&self.conn.get().unwrap())
    }

    pub fn read_unrecived_message(
        &self,
        userid: i32,
        message_index: i32,
    ) -> Option<Vec<message_model::Message>> {
        let t = message
            .filter(user_id.eq(&userid).and(id.lt(message_index)))
            .get_results::<message_model::Message>(&self.conn.get().unwrap());
        match t {
            Ok(messages) => Some(messages),
            Err(_) => None,
        }
    }
}
