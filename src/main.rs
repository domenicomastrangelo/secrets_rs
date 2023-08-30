use actix_web::{get, web, App, HttpServer, Responder};
use service_discovery::service_discovery::DB;
mod service_discovery;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start HTTP server
    HttpServer::new(|| App::new().service(users))
        .bind("127.0.0.1:8080")
        .expect("Can not bind to port 8080")
        .run()
        .await
}

#[get("/users/{user_id}")]
async fn users(user_id: web::Path<u32>) -> impl Responder {
    let services = service_discovery::service_discovery::Services::new();
    let service = services.get_service("db".to_string()).get();

    if let Some(db) = service.as_any().downcast_ref::<DB>() {
        let pool = db.get_pool().await;

        let user = user::user::get_user(user_id.into_inner(), pool).await;
        match user {
            Ok(user) => return web::Json(user),
            Err(_) => {
                return web::Json(user::user::User {
                    id: 0,
                    name: "Not found".to_string(),
                })
            }
        }
    } else {
        panic!("Failed to get database service");
    }
}
