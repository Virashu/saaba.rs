use crate::HTTPMethod;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub headers: HashMap<String, String>,
    pub path: String,
    pub method: HTTPMethod,
}

impl Request {
    pub fn new() -> Self {
        Request {
            headers: HashMap::new(),
            path: String::from(""),
            method: HTTPMethod::GET,
        }
    }
}
