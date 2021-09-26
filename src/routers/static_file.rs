use std::path::PathBuf;

use actix_http::error::ErrorForbidden;
use actix_web::{get,HttpRequest,Result};
use actix_files::NamedFile;

use crate::util::{check_token_expired::check_user_token_is_expired, get_token::get_token};
#[get("/avater/{filename:.*}")]
async fn get_avater(req:HttpRequest)->Result<NamedFile>{
    let token = get_token(&req);
    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap()) {
                Some(_) => {
                    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();
                    let dir=std::env::var("AVATER_DIR").unwrap();
                    let mut path=PathBuf::from(dir);
                    path.push(filename);
                   return  Ok(NamedFile::open(path)?)

                },
                None => return Err(ErrorForbidden("token expired")),
            }
        }
        None => return  Err(ErrorForbidden("not found token")),
    }
}

#[get("/binary/{filename:.*}")]
async fn get_binary_file(req:HttpRequest)->Result<NamedFile>{
    let token = get_token(&req);
    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap()) {
                Some(_) => {
                    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();
                    let dir=std::env::var("/static/").unwrap();
                    let mut path=PathBuf::from(dir);
                    path.push(filename);
                   return  Ok(NamedFile::open(path)?)

                },
                None => return Err(ErrorForbidden("token expired")),
            }
        }
        None => return  Err(ErrorForbidden("not found token")),
    }
}