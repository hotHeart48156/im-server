use actix_web::web;

use self::upload::upload_message_binarg;

pub mod user;
pub mod upload;
pub mod chat_server;
pub mod static_file;
pub fn scoped_function(cfg: &mut web::ServiceConfig) {
    cfg.service(user::login::login)
        .service(user::register::register)
        .service(user::friend::add_friend)
        .service(user::friend::delete_friend)
        .service(user::friend::list_friend)
        .service(user::room::add_room)
        .service(user::room::delete_room)
        .service(upload::upload_avater)
        .service(upload_message_binarg)
        .service(user::room::list_room)
        .service(static_file::get_avater)
        ;
}
