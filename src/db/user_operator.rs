use crate::models::user_model::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::{prelude::*, PgConnection, RunQueryDsl};

pub struct UserOperator<'a> {
    pub conn: &'a PgConnection,
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
            .get_result::<User>(self.conn.clone())
            .expect("error create user");
    }
    pub fn get_user_by_name(&self, user_name: String, user_password: String) -> User {
        let result: User = users
            .filter(name.eq(&user_name).and(password.eq(&user_password)))
            .limit(1)
            .get_result::<User>(self.conn.clone())
            .expect("error load user");
        result
    }

    pub fn get_user_by_id(&self, user_id: i32) -> User {
        let result: User = users
            .filter(id.eq(&user_id))
            .limit(1)
            .get_result::<User>(self.conn)
            .expect("error load user");
        result
    }
}
