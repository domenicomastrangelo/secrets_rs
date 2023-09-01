use actix_web::{get, web, App, HttpServer, Responder};
use service_discovery::db_service::DB;
use tracing::{debug, error};
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
    let service = match services.get_service("db".to_string()) {
        Some(service) => service,
        None => {
            error!("Failed to get database service");
            return web::Json(
                response::response::Response::new()
                    .set_success(false)
                    .set_message("Failed to get database service".to_string())
                    .build(),
            );
        }
    };

    if let Some(db) = service.as_any().downcast_ref::<DB>() {
        let inst = service_discovery::db_service::DB_INSTANCE.get();
        let inst = if inst.is_none() {
            DB::new().await
        } else {
            inst.unwrap_or(&db)
        };

        let pool = match inst.instance.clone() {
            Some(pool) => pool,
            None => {
                error!("Failed to get database instance");
                return web::Json(
                    response::response::Response::new()
                        .set_success(false)
                        .set_message("Failed to get database instance".to_string())
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
                debug!("Failed to get user: {:?}", e);
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
