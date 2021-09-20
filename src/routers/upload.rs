use actix_http::{error::ErrorInternalServerError, Error};
use actix_multipart::{Field, Multipart};
use actix_web::{post, web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

#[post("/upload_static_file")]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // payload.map(|f| {
    //     if let Ok(field) = f {
    //         web::block(|| {save(field)});
    //     }
    // });
    while let Ok(Some(field))=payload.try_next().await {
        let t=match save(field).await {
            Ok(ok) => {ok},
            Err(_) => {return Err(ErrorInternalServerError("not file name"))},
        };
    }
    Ok(HttpResponse::Ok().body("write ok"))
}

async fn save(mut field: Field) -> Result<Field, Error> {
    let content_type = match field.content_disposition() {
        Some(content) => content,
        None => return Err(ErrorInternalServerError("did`nt have content type")),
    };
    let filename = match content_type.get_filename() {
        Some(name) => name.to_string(),
        None => return Err(ErrorInternalServerError("not file name")),
    };
    let filedir = "./static";
    let mut file = match std::fs::File::create(format!("{}/{}", filedir, filename)) {
        Ok(file) => file,
        Err(e) => return Err(ErrorInternalServerError(e)),
    };
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // filesystem operations are blocking, we have to use threadpool
        file = web::block(move || file.write_all(&data).map(|_| file)).await?;
    }
    Ok(field)
}
