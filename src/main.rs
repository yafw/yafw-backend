use actix_web::{web, App, HttpServer};
mod handlers;
mod statistics;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    statistics::statistics_thread_init();
    
    HttpServer::new( || {
        App::new()
            // System
            .route("/system/sysctl/{cmd}", web::get().to( handlers::system::get_sysctl ))
            .route("/system/cpu", web::get().to( handlers::system::get_cpu ))
            
            // Network
            .route("/network/interfaces", web::get().to( handlers::network::get_interfaces ))
            .route("/network/interface/{device}", web::get().to( handlers::network::get_interface ))

            // Config
            .route("/config/save", web::get().to( handlers::config::save ))
            .route("/config/load", web::get().to( handlers::config::load ))
            .route("/config/commit", web::get().to( handlers::config::commit ))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
