use actix_web::{HttpRequest, HttpResponse};
use std::process::Command;
use std::fs;

pub async fn get_sysctl(req: HttpRequest) -> HttpResponse {
    let cmd = req.match_info().get("cmd").unwrap_or("unknown");

    let output = Command::new("sysctl")
        .arg("-n")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    let output_stdout = String::from_utf8_lossy(&output.stdout);
    
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Server", "yafw-backend"))
        .body( format!("\"{}\"", output_stdout.trim()) )
}

pub async fn get_cpu(req: HttpRequest) -> HttpResponse {

    let proc_cpu = fs::read_to_string("/proc/stat").expect("Error reading /proc/cpu");
    
    //https://stackoverflow.com/questions/9229333/how-to-get-overall-cpu-usage-e-g-57-on-linux



    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Server", "yafw-backend"))
        .body( format!("{}", proc_cpu) )
}
