use super::db_service::DB;
use std::any::Any;

pub trait Service: Any {
    fn get(&self) -> Box<dyn Service>;
    fn get_name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub struct Services {
    services: Vec<Box<dyn Service>>,
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
