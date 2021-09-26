use actix_http::http::HeaderValue;
use actix_web::HttpRequest;

pub fn get_token(req: &HttpRequest)->Option<&HeaderValue>{
  let tk=std::env::var("TOKEN_PARAMAS").unwrap();
  req.headers().get(tk)
}