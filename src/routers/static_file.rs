use std::path::PathBuf;

use actix_web::{get,HttpRequest,Result};
use actix_files::NamedFile;
#[get("/get_avater")]
async fn get_avater(req:HttpRequest)->Result<NamedFile>{
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("{:?}",path);
    Ok(NamedFile::open(path)?)
}