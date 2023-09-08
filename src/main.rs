use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[get("/api/ping")]
async fn ping() -> impl Responder {
    const MESSAGE: &str = "Ping successfull";
    HttpResponse::Ok().json(
        serde_json::json!(
            {
                "status": "success",
                "message": MESSAGE
            }
        )
    )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(ping)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
