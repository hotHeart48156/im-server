use actix_web::web;

pub mod user;
pub mod upload;
pub mod chat_server;
pub fn scoped_function(cfg: &mut web::ServiceConfig) {
    cfg.service(user::login::login)
        .service(user::register::register)
        .service(user::friend::add_friend)
        .service(user::friend::delete_friend)
        .service(user::friend::list_friend)
        .service(user::room::add_room)
        .service(user::room::delete_room)
        .service(upload::upload)
        .service(user::room::list_room);
}
