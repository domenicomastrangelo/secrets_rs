use once_cell::sync::OnceCell;
use std::any::Any;
use tracing::error;

pub trait Service: Any {
    fn get(&self) -> Box<dyn Service>;
    fn get_name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub struct DB {
    pub name: String,
    pub instance: Option<sqlx::mysql::MySqlPool>,
}

impl Service for DB {
    fn get(&self) -> Box<dyn Service> {
        Box::new(DB {
            name: "db".to_string(),
            instance: None,
        })
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Services {
    services: Vec<Box<dyn Service>>,
}

pub static DB_INSTANCE: OnceCell<&'static DB> = OnceCell::new();

impl DB {
    pub async fn new() -> &'static DB {
        match DB_INSTANCE.get() {
            Some(db) => db,
            None => {
                let mut db = DB {
                    name: "db".to_string(),
                    instance: None,
                };

                db.get_pool().await;

                DB_INSTANCE
                    .set(Box::leak(Box::new(db)))
                    .unwrap_or_else(|_| panic!("Failed to set database instance"));

                match DB_INSTANCE.get() {
                    Some(db) => db,
                    None => panic!("Failed to get database instance"),
                }
            }
        }
    }

    async fn get_pool(&mut self) {
        println!("Getting database pool");

        let connection_string = format!(
            "mysql://{}:{}@{}:{}/{}",
            std::env::var("USER").unwrap_or("".to_string()),
            std::env::var("PASSWORD").unwrap_or("".to_string()),
            std::env::var("HOST").unwrap_or("".to_string()),
            std::env::var("PORT").unwrap_or("".to_string()),
            std::env::var("DATABASE").unwrap_or("".to_string()),
        );

        self.instance = match sqlx::mysql::MySqlPool::connect(&connection_string).await {
            Ok(pool) => Some(pool),
            Err(e) => {
                error!("Failed to get database pool: {:?}", e);
                None
            }
        };
    }
}

impl Services {
    pub fn new() -> Services {
        Services {
            services: vec![Box::new(DB {
                name: "db".to_string(),
                instance: None,
            })],
        }
    }

    pub fn get_service(&self, name: String) -> Option<&Box<dyn Service>> {
        for service in &self.services {
            if service.get_name() == name.as_str() {
                return Some(service);
            }
        }

        None
    }
}
