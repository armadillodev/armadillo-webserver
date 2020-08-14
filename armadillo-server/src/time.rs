pub fn now() -> u64 {
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_secs()
}

use actix_web::{Responder, HttpResponse};

pub async fn time_route() -> impl Responder {
    HttpResponse::Ok().body(now().to_string())
}
