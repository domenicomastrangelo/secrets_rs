use actix_web::{get, web, App, HttpServer, Responder};
use service_discovery::service_discovery::DB;
use tracing::error;
mod response;
mod service_discovery;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    tracing_subscriber::fmt::format()
        .with_line_number(true)
        .with_file(true)
        .with_source_location(true);

    // Start HTTP server
    HttpServer::new(|| App::new().service(users))
        .bind("127.0.0.1:8080")
        .expect("Can not bind to port 8080")
        .run()
        .await
}

#[get("/users/{user_id}")]
#[tracing::instrument]
async fn users(user_id: web::Path<u32>) -> impl Responder {
    let services = service_discovery::service_discovery::Services::new();
    let service = services.get_service("db".to_string()).get();

    if let Some(db) = service.as_any().downcast_ref::<DB>() {
        let pool = db.get_pool().await;

        let pool = match pool {
            Ok(pool) => pool,
            Err(e) => {
                error!("Failed to get database pool: {:?}", e);
                return web::Json(
                    response::response::Response::new()
                        .set_success(false)
                        .set_message("Failed to get database pool".to_string())
                        .build(),
                );
            }
        };

        let user = user::user::get_user(user_id.into_inner(), pool).await;
        match user {
            Ok(user) => {
                return web::Json(response::response::Response::new().set_data(user).build())
            }
            Err(e) => {
                println!("Failed to get user: {:?}", e);
                return web::Json(
                    response::response::Response::new()
                        .set_success(false)
                        .set_message("Failed to get user".to_string())
                        .build(),
                );
            }
        }
    } else {
        error!("Failed to cast database service");

        return web::Json(
            response::response::Response::new()
                .set_success(false)
                .set_message("Failed to get database service".to_string())
                .build(),
        );
    }
}