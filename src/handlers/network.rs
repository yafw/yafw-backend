use actix_web::{HttpResponse, HttpRequest};
use std::process::Command;

pub async fn get_interfaces(req: HttpRequest) -> HttpResponse {

    let output = Command::new("ip")
        .arg("-j")
        .arg("link")
        .output()
        .expect("failed to execute process");

    let output_stdout = String::from_utf8_lossy(&output.stdout);
    
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Server", "yafw-backend"))
        .body( format!("{}", output_stdout.trim()) )
}

pub async fn get_interface(req: HttpRequest) -> HttpResponse {
    let device = req.match_info().get("device").unwrap_or("unknown");

    let output = Command::new("ip")
        .arg("-j")
        .arg("link")
        .arg("show")
        .arg(device)
        .output()
        .expect("failed to execute process");

    let output_stdout = String::from_utf8_lossy(&output.stdout);
    
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Server", "yafw-backend"))
        .body( format!("{}", output_stdout.trim()) )
}
