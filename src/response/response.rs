use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

impl<T: Serialize + Deserialize<'static> + Default + Clone> Response<T> {
    pub fn new() -> Response<T> {
        Response {
            success: true,
            message: "".to_string(),
            data: T::default(),
        }
    }

    pub fn set_success(&mut self, success: bool) -> &mut Response<T> {
        self.success = success;

        self
    }

    pub fn set_message(&mut self, message: String) -> &mut Response<T> {
        self.message = message;

        self
    }

    pub fn set_data(&mut self, data: T) -> &mut Response<T> {
        self.data = data;

        self
    }

    pub fn build(&self) -> Response<T> {
        Response {
            success: self.success,
            message: self.message.as_str().to_string(),
            data: self.data.clone(),
        }
    }
}
