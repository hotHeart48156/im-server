use crate::models::user_model::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::server::DbPoolType;
use diesel::{prelude::*, RunQueryDsl};

pub struct UserOperator<'a> {
    pub conn:&'a DbPoolType
}

impl UserOperator<'_> {
    pub fn new_user(&self, user_name: &str, user_password: &str, user_gender: i16) {
        let new_user = NewUser {
            name: user_name,
            password: user_password,
            gender: user_gender,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&self.conn.get().unwrap())
            .expect("error create user");
    }
    pub fn get_user_by_name_and_password(
        &self,
        user_name: String,
        user_password: String,
    ) -> Option<User> {
        let result: User = users
            .filter(name.eq(&user_name).and(password.eq(&user_password)))
            .limit(1)
            .get_result::<User>(&self.conn.get().unwrap())
            .expect("error load user");
        if result.id == 0 {
            return None;
        } else {
            return Some(result);
        }
    }

    pub fn get_user_by_id(&self, user_id: i32) -> User {
        let result: User = users
            .filter(id.eq(&user_id))
            .limit(1)
            .get_result::<User>(&self.conn.get().unwrap())
            .expect("error load user");
        result
    }
}
