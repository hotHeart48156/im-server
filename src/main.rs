use im_server::server::start_server;
// use windows_service::service_dispatcher;
// #[macro_use]
// extern crate windows_service;
// use std::ffi::OsString;

// define_windows_service!(ffi_service_main, my_service_main);

// fn my_service_main(arguments: Vec<OsString>) {
//     // The entry point where execution will start on a background thread after a call to
//     // `service_dispatcher::start` from `main`.
// }
#[actix_web::main]

async fn main() {
    // service_dispatcher::start("myservice", ffi_service_main).unwrap();
    start_server().await;
}
