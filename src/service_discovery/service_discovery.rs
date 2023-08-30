use std::any::Any;

pub trait Service: Any {
    fn get(&self) -> Box<dyn Service>;
    fn get_name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub struct DB {
    name: String,
}

impl Service for DB {
    fn get(&self) -> Box<dyn Service> {
        Box::new(DB {
            name: "db".to_string(),
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

impl DB {
    pub async fn get_pool(&self) -> sqlx::mysql::MySqlPool {
        println!("Getting database pool");

        let connection_string = format!(
            "mysql://{:?}:{:?}@{:?}:3306/{:?}",
            std::env::var("USER"),
            std::env::var("PASSWORD"),
            std::env::var("HOST"),
            std::env::var("DATABASE_NAME")
        );

        match sqlx::mysql::MySqlPool::connect(&connection_string).await {
            Ok(pool) => pool,
            Err(_) => {
                println!("Failed to connect to database");
                panic!();
            }
        }
    }
}

impl Services {
    pub fn new() -> Services {
        Services {
            services: vec![Box::new(DB {
                name: "db".to_string(),
            })],
        }
    }

    pub fn get_service(&self, name: String) -> &Box<dyn Service> {
        for service in &self.services {
            if service.get_name() == name.as_str() {
                return service;
            }
        }

        panic!("Service not found");
    }
}
