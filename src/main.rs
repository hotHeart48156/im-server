use im_server::server::start_server;

#[actix_web::main]

async fn main() {
    start_server().await;
    // let tt="/send_message_to_friend/hello/88997676";
    // let mut c=tt.splitn(4,"/");
    // c.next();
    // println!("{:?}",c.next());
}
