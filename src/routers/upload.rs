use actix_http::{error::ErrorInternalServerError, Error};
use actix_multipart::{Field, Multipart};
use actix_web::{HttpRequest, HttpResponse, post, web};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use crypto::{digest::Digest, md5::Md5};
use crate::{db::user_operator::UserOperator, server::DbPoolType, util::{check_token_expired::check_user_token_is_expired, get_token::get_token}};

#[post("/upload_message_binary")]
pub async fn upload_message_binarg(mut payload: Multipart,req:HttpRequest) -> Result<HttpResponse, Error> {
    let token = get_token(&req);

    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap()) {
                Some(id) => {

                    let dir=match std::env::var("MESSAGE_BINARY_DIR") {
                        Ok(path) => {path},
                        Err(_) => return Err(ErrorInternalServerError("not file name")),
                    };
                    while let Ok(Some(field))=payload.try_next().await {
                        let dir=dir.clone();
                        match save(field,dir,id.to_owned()).await {
                            Ok(ok) => {ok},
                            Err(_) => {return Err(ErrorInternalServerError("not file name"))},
                        };
                    }

                },
                None => return Ok(HttpResponse::Forbidden().body("token is expired")),
            }
        }
        None => return Ok(HttpResponse::Forbidden().body("token is expired")),
    }
    
    Ok(HttpResponse::Ok().body("write ok"))  
}
       
    
   

 



#[post("/upload_avater_image")]
pub async fn upload_avater(mut payload: Multipart,req:HttpRequest,pool: web::Data<DbPoolType>,) -> Result<HttpResponse, Error> {
   
    let token = get_token(&req);
    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap()) {
                Some(id) => {
                    let dir=match std::env::var("AVATER_DIR") {
                        Ok(path) => {path},
                        Err(_) => return Err(ErrorInternalServerError("not file name")),
                    };
                    let mut filename=String::from("");
                    while let Ok(Some(field))=payload.try_next().await {
                        let dir=dir.clone();
                        filename=match save(field,dir,id.clone()).await {
                            Ok(filename) => {filename},
                            Err(_) => {return Err(ErrorInternalServerError("not file name"))},
                        };
                    }
                    UserOperator{conn:&pool}.add_avater(id.parse::<i32>().unwrap(), filename).unwrap();
                    Ok(HttpResponse::Ok().body("write ok"))

                },
                None => return Ok(HttpResponse::Forbidden().body("token is expired")),
            }
        }
        None => return Ok(HttpResponse::Forbidden().body("token is expired")),
    }
   
   
}


async fn save(mut field: Field,path:String,userid:String) -> Result<String, Error> {
    let content_type = match field.content_disposition() {
        Some(content) => content,
        None => return Err(ErrorInternalServerError("did`nt have content type")),
    };
    let filename = match content_type.get_filename() {
        Some(name) => {
            let mut splited=name.split(".");
            splited.next();
            let addr=match splited.next() {
                Some(att) => {att},
                None => {""},
            };
            let mut hasher = Md5::new();
            hasher.input_str(&name);
            format!("{}{}.{}",hasher.result_str(),userid,addr)
        },
        None => return Err(ErrorInternalServerError("not file name")),
    };
    let filedir =path.as_str();
    if !std::path::Path::new(&filedir).exists() {
        std::fs::create_dir(filedir)?;
    }
    let mut file = match std::fs::File::create(format!("{}/{}", filedir, filename)) {
        Ok(file) => file,
        Err(e) =>{ 
            
            return Err(ErrorInternalServerError(e))
        },
    };
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // filesystem operations are blocking, we have to use threadpool
        file = web::block(move || file.write_all(&data).map(|_| file)).await?;
    }
    Ok(filename)
}


pub fn handle<F>(req: HttpRequest, handle: F) -> Result<HttpResponse, Error>
where
    F: Fn(String) -> Result<HttpResponse, Error>,
{
    let token = get_token(&req);
    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap()) {
                Some(id) => handle(id),
                None => Ok(HttpResponse::Forbidden().body("token is expired")),
            }
        }
        None => Ok(HttpResponse::Forbidden().body("token is expired")),
    }
}

