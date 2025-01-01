use crate::HTTPMethod;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: HTTPMethod::default(),
            url: String::new(),
            headers: HashMap::new(),
        }
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}
