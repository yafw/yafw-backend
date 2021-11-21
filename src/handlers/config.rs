use actix_web::{HttpRequest, HttpResponse};

pub async fn save(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Server", "yafw-backend"))
        .body( "N/A" )
}

pub async fn load(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Server", "yafw-backend"))
        .body( "N/A" )
}

pub async fn commit(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Server", "yafw-backend"))
        .body( "N/A" )
}
