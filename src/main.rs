mod users;
mod config;
mod ping;

use config::entities::{ApplicationConfig, DatabaseConfig};

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: ApplicationConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let app_config = ApplicationConfig::init();
    let db_config = DatabaseConfig::init();

    let pool = match PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .connect(&db_config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(
                vec![
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                ]
            )
            .supports_credentials();
        App::new()
            .app_data(
                web::Data::new(
                    AppState {
                        db: pool.clone(),
                        env: app_config.clone()
                    }
                )
            )
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(users::controllers::register)
                    .service(users::controllers::login)
                    .service(users::controllers::logout)
                    .service(users::controllers::profile)
                    .service(ping::controllers::ping)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
