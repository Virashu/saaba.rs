use crate::HTTPMethod;

pub struct Request {
    pub headers: Vec<String>,
    pub path: String,
    pub method: HTTPMethod,
}

impl Request {
    pub fn new() -> Self {
        Request {
            headers: Vec::new(),
            path: String::from(""),
            method: HTTPMethod::GET,
        }
    }
}
