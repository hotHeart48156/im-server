use actix_http::http::HeaderValue;
use actix_web::HttpRequest;

pub fn get_token(req: &HttpRequest)->Option<&HeaderValue>{
  req.headers().get("token")
}