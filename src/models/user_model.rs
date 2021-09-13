// use diesel:
use crate::schema::users;
use serde::{Serialize,Deserialize};
#[derive(Queryable,AsExpression,Serialize,Deserialize,Debug,Clone)]
pub struct User {
    pub id: i32 ,
    pub name:String ,
    pub gender:Option<i16> ,
    pub password: String ,
}


#[derive(Insertable,AsExpression)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub gender: i16,
    pub password: &'a str,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PostUser{
    pub name:String ,
    pub gender:Option<i16> ,
    pub password: String ,
}